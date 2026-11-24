use pilota::{
    Bytes, BytesMut, LinkedBytes,
    thrift::{
        TAsyncCompactProtocol, TAsyncInputProtocol, TInputProtocol, TLengthProtocol,
        TOutputProtocol, TStructIdentifier, TType,
        compact::{TCompactInputProtocol, TCompactOutputProtocol},
    },
};

#[tokio::test]
async fn compact_field_id_overflow_is_an_error() {
    let mut p = TAsyncCompactProtocol::new(&[5, 0xfe, 0xff, 3, 0, 0x15][..]);
    p.read_struct_begin().await.unwrap();
    assert_eq!(p.read_field_begin().await.unwrap().id, Some(i16::MAX));
    p.read_i32().await.unwrap();
    assert!(p.read_field_begin().await.is_err());
}

#[test]
fn sync_compact_field_id_overflow_is_an_error() {
    let mut bytes = Bytes::from_static(&[5, 0xfe, 0xff, 3, 0, 0x15]);
    let mut p = TCompactInputProtocol::new(&mut bytes);
    p.read_struct_begin().unwrap();
    assert_eq!(p.read_field_begin().unwrap().id, Some(i16::MAX));
    p.read_i32().unwrap();
    assert!(p.read_field_begin().is_err());
}

#[tokio::test]
async fn compact_bool_field_overflow_preserves_state() {
    for (header, next_bool) in [(0x11, false), (0x12, true)] {
        // Field i16::MAX, an overflowing bool header, then a standalone bool.
        let data = [5, 0xfe, 0xff, 3, 0, header, if next_bool { 1 } else { 2 }];
        let mut p = TAsyncCompactProtocol::new(&data[..]);
        p.read_struct_begin().await.unwrap();
        assert_eq!(p.read_field_begin().await.unwrap().id, Some(i16::MAX));
        p.read_i32().await.unwrap();
        assert!(p.read_field_begin().await.is_err());
        assert_eq!(p.read_bool().await.unwrap(), next_bool);
        assert!(p.read_byte().await.is_err());
    }
}

#[test]
fn sync_compact_bool_field_overflow_preserves_state() {
    for (header, next_bool) in [(0x11, false), (0x12, true)] {
        let data = [5, 0xfe, 0xff, 3, 0, header, if next_bool { 1 } else { 2 }];
        let mut bytes = Bytes::copy_from_slice(&data);
        let mut p = TCompactInputProtocol::new(&mut bytes);
        p.read_struct_begin().unwrap();
        assert_eq!(p.read_field_begin().unwrap().id, Some(i16::MAX));
        p.read_i32().unwrap();
        assert!(p.read_field_begin().is_err());
        assert_eq!(p.read_bool().unwrap(), next_bool);
        assert!(p.read_byte().is_err());
    }
}

#[tokio::test]
async fn compact_truncated_bool_field_id_preserves_state() {
    for header in [1, 2] {
        // A bool header with no delta must be followed by an explicit field ID.
        let data = [header, 0x80];
        let mut p = TAsyncCompactProtocol::new(&data[..]);
        assert!(p.read_field_begin().await.is_err());
        assert!(p.read_bool().await.is_err());
    }
}

#[test]
fn sync_compact_truncated_bool_field_id_preserves_state() {
    for header in [1, 2] {
        let mut bytes = Bytes::copy_from_slice(&[header, 0x80]);
        let mut p = TCompactInputProtocol::new(&mut bytes);
        assert!(p.read_field_begin().is_err());
        assert!(p.read_bool().is_err());
    }
}

const EXTREME_FIELD_IDS: [i16; 3] = [i16::MAX, i16::MIN, i16::MAX];
const EXTREME_FIELD_CASES: [(TType, &[u8]); 2] = [
    (
        TType::I32,
        &[
            5, 0xfe, 0xff, 3, 0, 5, 0xff, 0xff, 3, 0, 5, 0xfe, 0xff, 3, 0, 0,
        ],
    ),
    (
        TType::Bool,
        &[1, 0xfe, 0xff, 3, 2, 0xff, 0xff, 3, 1, 0xfe, 0xff, 3, 0],
    ),
];

fn write_extreme_fields(p: &mut impl TOutputProtocol, field_type: TType) {
    p.write_struct_begin(&TStructIdentifier::new("extreme_fields"))
        .unwrap();
    for id in EXTREME_FIELD_IDS {
        p.write_field_begin(field_type, id).unwrap();
        if field_type == TType::Bool {
            p.write_bool(id == i16::MAX).unwrap();
        } else {
            p.write_i32(0).unwrap();
        }
        p.write_field_end().unwrap();
    }
    p.write_field_stop().unwrap();
    p.write_struct_end().unwrap();
}

async fn assert_extreme_fields_round_trip(data: &[u8], field_type: TType) {
    let mut bytes = Bytes::copy_from_slice(data);
    let mut sync = TCompactInputProtocol::new(&mut bytes);
    let mut asynchronous = TAsyncCompactProtocol::new(data);
    sync.read_struct_begin().unwrap();
    asynchronous.read_struct_begin().await.unwrap();
    for id in EXTREME_FIELD_IDS {
        let sync_field = sync.read_field_begin().unwrap();
        let async_field = asynchronous.read_field_begin().await.unwrap();
        assert_eq!(sync_field.id, Some(id));
        assert_eq!(sync_field.field_type, field_type);
        assert_eq!(async_field, sync_field);
        if field_type == TType::Bool {
            assert_eq!(sync.read_bool().unwrap(), id == i16::MAX);
            assert_eq!(asynchronous.read_bool().await.unwrap(), id == i16::MAX);
        } else {
            assert_eq!(sync.read_i32().unwrap(), 0);
            assert_eq!(asynchronous.read_i32().await.unwrap(), 0);
        }
        sync.read_field_end().unwrap();
        asynchronous.read_field_end().await.unwrap();
    }
    assert_eq!(sync.read_field_begin().unwrap().field_type, TType::Stop);
    assert_eq!(
        asynchronous.read_field_begin().await.unwrap().field_type,
        TType::Stop
    );
    sync.read_struct_end().unwrap();
    asynchronous.read_struct_end().await.unwrap();
    assert!(sync.read_byte().is_err());
    assert!(asynchronous.read_byte().await.is_err());
}

#[tokio::test]
async fn compact_bytesmut_round_trips_extreme_field_ids() {
    for (field_type, expected) in EXTREME_FIELD_CASES {
        let mut bytes = BytesMut::new();
        write_extreme_fields(
            &mut TCompactOutputProtocol::new(&mut bytes, false),
            field_type,
        );
        assert_eq!(bytes.as_ref(), expected);
        assert_extreme_fields_round_trip(&bytes, field_type).await;
    }
}

#[tokio::test]
async fn compact_linkedbytes_round_trips_extreme_field_ids() {
    for (field_type, expected) in EXTREME_FIELD_CASES {
        let mut bytes = LinkedBytes::new();
        write_extreme_fields(
            &mut TCompactOutputProtocol::new(&mut bytes, false),
            field_type,
        );
        assert_eq!(bytes.bytes().as_ref(), expected);
        assert_extreme_fields_round_trip(bytes.bytes(), field_type).await;
    }
}

fn extreme_fields_len(p: &mut impl TLengthProtocol, field_type: TType) -> usize {
    let mut len = p.struct_begin_len(&TStructIdentifier::new("extreme_fields"));
    for id in EXTREME_FIELD_IDS {
        len += p.field_begin_len(field_type, Some(id));
        len += if field_type == TType::Bool {
            p.bool_len(id == i16::MAX)
        } else {
            p.i32_len(0)
        };
        len += p.field_end_len();
    }
    len += p.field_stop_len();
    len + p.struct_end_len()
}

#[test]
fn compact_output_lengths_match_extreme_field_ids() {
    for (field_type, expected) in EXTREME_FIELD_CASES {
        let mut p = TCompactOutputProtocol::new((), false);
        assert_eq!(extreme_fields_len(&mut p, field_type), expected.len());
    }
}

#[test]
fn compact_input_lengths_match_extreme_field_ids() {
    for (field_type, expected) in EXTREME_FIELD_CASES {
        let mut p = TCompactInputProtocol::new(());
        assert_eq!(extreme_fields_len(&mut p, field_type), expected.len());
    }
}
