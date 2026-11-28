use pilota::thrift::{TAsyncBinaryProtocol, TAsyncCompactProtocol, TAsyncInputProtocol, TType};
use tokio::io::AsyncReadExt;

fn assert_size_limit(error: pilota::thrift::ThriftException) {
    assert!(
        matches!(error, pilota::thrift::ThriftException::Protocol(ref error)
        if error.kind() == pilota::thrift::ProtocolExceptionKind::SizeLimit),
        "{error:?}"
    );
}

#[tokio::test]
async fn fixed_fields_varints_headers_and_stop_report_size_limit() {
    let mut binary = TAsyncBinaryProtocol::with_read_limit(&[0; 8][..], 3);
    assert_size_limit(binary.read_i64().await.unwrap_err());
    let mut binary = TAsyncBinaryProtocol::with_read_limit(&[0][..], 0);
    assert_size_limit(binary.skip(TType::Struct).await.unwrap_err());
    let mut compact = TAsyncCompactProtocol::with_read_limit(&[0x80, 1][..], 1);
    assert_size_limit(compact.read_i32().await.unwrap_err());
    let mut compact = TAsyncCompactProtocol::with_read_limit(&[0][..], 0);
    assert_size_limit(compact.skip(TType::Struct).await.unwrap_err());
    let mut binary = TAsyncBinaryProtocol::with_read_limit(&[0, 0, 0, 2][..], 4);
    assert_size_limit(binary.read_bytes_vec().await.unwrap_err());
    let mut compact = TAsyncCompactProtocol::with_read_limit(&[2][..], 1);
    assert_size_limit(compact.read_bytes_vec().await.unwrap_err());
    // A real EOF stays a transport error even when a budget is configured.
    let mut binary = TAsyncBinaryProtocol::with_read_limit(&[][..], 1);
    assert!(matches!(
        binary.read_byte().await.unwrap_err(),
        pilota::thrift::ThriftException::Transport(_)
    ));
    assert!(matches!(
        pilota::thrift::ThriftException::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "reader error"
        )),
        pilota::thrift::ThriftException::Transport(_)
    ));
}

#[test]
fn binary_skip_does_not_embed_the_discard_buffer_in_recursive_futures() {
    let mut binary = TAsyncBinaryProtocol::new(&[][..]);
    let binary_size = std::mem::size_of_val(&binary.skip_binary());
    let mut compact = TAsyncCompactProtocol::new(&[][..]);
    let compact_size = std::mem::size_of_val(&compact.skip_binary());
    assert!(
        binary_size < 512 && compact_size < 512,
        "binary={binary_size}, compact={compact_size}"
    );
}

#[tokio::test]
async fn truncated_large_fields_only_request_a_small_initial_buffer() {
    use std::{
        pin::Pin,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, ReadBuf};
    struct Observed<'a>(&'a [u8], Arc<AtomicUsize>);
    impl AsyncRead for Observed<'_> {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            self.1.fetch_max(buf.remaining(), Ordering::Relaxed);
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }
    for compact in [false, true] {
        let header: &[u8] = if compact {
            &[0x80, 0x80, 0x80, 1]
        } else {
            &[0, 0x20, 0, 0]
        };
        let largest = Arc::new(AtomicUsize::new(0));
        let reader = Observed(header, largest.clone());
        let result = if compact {
            TAsyncCompactProtocol::with_read_limit(reader, 4 * 1024 * 1024)
                .read_bytes_vec()
                .await
        } else {
            TAsyncBinaryProtocol::with_read_limit(reader, 4 * 1024 * 1024)
                .read_bytes_vec()
                .await
        };
        assert!(result.is_err());
        assert!(largest.load(Ordering::Relaxed) <= 64 * 1024);
    }
}

#[tokio::test]
async fn large_fields_keep_the_following_message_unread() {
    let value = vec![42; 128 * 1024];
    for compact in [false, true] {
        let mut input = if compact {
            vec![0x80, 0x80, 8]
        } else {
            vec![0, 2, 0, 0]
        };
        input.extend_from_slice(&value);
        input.push(99);
        if compact {
            let mut p = TAsyncCompactProtocol::new(input.as_slice());
            assert_eq!(p.read_bytes_vec().await.unwrap(), value);
            assert_eq!(p.read_byte().await.unwrap(), 99);
        } else {
            let mut p = TAsyncBinaryProtocol::new(input.as_slice());
            assert_eq!(p.read_bytes_vec().await.unwrap(), value);
            assert_eq!(p.read_byte().await.unwrap(), 99);
        }
    }
}

#[tokio::test]
async fn negative_binary_length_returns_an_error() {
    let mut protocol = TAsyncBinaryProtocol::new(&b"\xff\xff\xff\xff"[..]);
    assert!(protocol.read_bytes_vec().await.is_err());
}

#[tokio::test]
async fn skipping_uuid_preserves_the_following_value() {
    let data = [7_u8; 17];
    let mut protocol = TAsyncBinaryProtocol::new(&data[..]);
    protocol.skip(TType::Uuid).await.unwrap();
    assert_eq!(protocol.read_byte().await.unwrap(), 7);
}

#[tokio::test]
async fn binary_lengths_are_checked_before_allocation() {
    for size in [-1_i32, i32::MAX] {
        let header = size.to_be_bytes();
        let mut p = TAsyncBinaryProtocol::with_read_limit(header.as_slice(), 16);
        assert!(p.read_bytes_vec().await.is_err());
        let list = [vec![TType::I32 as u8], header.to_vec()].concat();
        let mut p = TAsyncBinaryProtocol::with_read_limit(list.as_slice(), 16);
        assert!(p.read_list_begin().await.is_err());
        let mut p = TAsyncBinaryProtocol::with_read_limit(list.as_slice(), 16);
        assert!(p.read_set_begin().await.is_err());
        let map = [vec![TType::I32 as u8, TType::I64 as u8], header.to_vec()].concat();
        let mut p = TAsyncBinaryProtocol::with_read_limit(map.as_slice(), 16);
        assert!(p.read_map_begin().await.is_err());
    }
}

#[tokio::test]
async fn compact_lengths_are_checked_before_allocation() {
    let size = &[0xff, 0xff, 0xff, 0xff, 0x0f][..];
    let mut p = TAsyncCompactProtocol::with_read_limit(size, 32);
    assert!(p.read_bytes_vec().await.is_err());
    let list = [vec![0xf5], size.to_vec()].concat();
    let mut p = TAsyncCompactProtocol::with_read_limit(list.as_slice(), 32);
    assert!(p.read_list_begin().await.is_err());
    let mut p = TAsyncCompactProtocol::with_read_limit(list.as_slice(), 32);
    assert!(p.read_set_begin().await.is_err());
    let map = [size.to_vec(), vec![0x55]].concat();
    let mut p = TAsyncCompactProtocol::with_read_limit(map.as_slice(), 32);
    assert!(p.read_map_begin().await.is_err());
}

#[tokio::test]
async fn skip_binary_does_not_require_utf8_and_keeps_prefetched_next_message() {
    // A binary field containing arbitrary bytes, then STOP and a subsequent message
    // byte.
    let binary = [11, 0, 1, 0, 0, 0, 2, 0xff, 0xfe, 0, 42];
    let compact = [0x18, 2, 0xff, 0xfe, 0, 42];
    for (bytes, compact_protocol) in [(binary.as_slice(), false), (compact.as_slice(), true)] {
        let mut reader = tokio::io::BufReader::new(bytes);
        let limit = bytes.len() - 1;
        if compact_protocol {
            let mut p = TAsyncCompactProtocol::with_read_limit(&mut reader, limit);
            p.skip(TType::Struct).await.unwrap();
            assert_eq!(p.remaining_read_limit(), Some(0));
            assert!(p.read_byte().await.is_err());
        } else {
            let mut p = TAsyncBinaryProtocol::with_read_limit(&mut reader, limit);
            p.skip(TType::Struct).await.unwrap();
            assert_eq!(p.remaining_read_limit(), Some(0));
            assert!(p.read_byte().await.is_err());
        }
        assert_eq!(reader.read_u8().await.unwrap(), 42);
    }
}

#[tokio::test]
async fn one_byte_short_limit_and_truncated_field_are_errors() {
    let mut p = TAsyncBinaryProtocol::with_read_limit(&[11, 0, 1, 0, 0, 0, 1, 9, 0][..], 8);
    assert!(p.skip(TType::Struct).await.is_err());
    let mut p = TAsyncCompactProtocol::with_read_limit(&[0x18, 1, 9, 0][..], 3);
    assert!(p.skip(TType::Struct).await.is_err());
    let mut p = TAsyncBinaryProtocol::with_read_limit(&[0, 0, 0, 2, 1][..], 32);
    assert!(p.skip_binary().await.is_err());
    let mut p = TAsyncCompactProtocol::with_read_limit(&[2, 1][..], 32);
    assert!(p.skip_binary().await.is_err());
}

#[tokio::test]
async fn invalid_utf8_strings_are_errors() {
    let mut binary = TAsyncBinaryProtocol::new(&[0, 0, 0, 1, 0xff][..]);
    assert!(binary.read_string().await.is_err());
    let mut compact = TAsyncCompactProtocol::new(&[1, 0xff][..]);
    assert!(compact.read_string().await.is_err());
}

#[tokio::test]
async fn compact_nested_struct_restores_parent_field_id() {
    let mut p = TAsyncCompactProtocol::with_read_limit(&[0x1c, 0x45, 2, 0, 0x15, 4, 0][..], 7);
    p.read_struct_begin().await.unwrap();
    assert_eq!(p.read_field_begin().await.unwrap().id, Some(1));
    p.skip(TType::Struct).await.unwrap();
    assert_eq!(p.read_field_begin().await.unwrap().id, Some(2));
    assert_eq!(p.read_i32().await.unwrap(), 2);
    assert_eq!(p.read_field_begin().await.unwrap().field_type, TType::Stop);
    p.read_struct_end().await.unwrap();
}

#[tokio::test]
async fn skip_preserves_existing_depth_limit() {
    let bytes = [[12, 0, 1].repeat(65), vec![0; 66]].concat();
    let mut p = TAsyncBinaryProtocol::with_read_limit(bytes.as_slice(), bytes.len());
    assert!(p.skip(TType::Struct).await.is_err());
    let bytes = [vec![0x1c; 65], vec![0; 66]].concat();
    let mut p = TAsyncCompactProtocol::with_read_limit(bytes.as_slice(), bytes.len());
    assert!(p.skip(TType::Struct).await.is_err());
}
