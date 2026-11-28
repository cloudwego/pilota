//! Generated Thrift message and RPC payload encoding contracts.
use pilota::{
    Bytes, LinkedBytes,
    thrift::{Message, binary::TBinaryProtocol},
};

include!("../test_data/thrift/payload.rs");
include!("../test_data/thrift/backend_types.rs");
use payload::{payload_service::*, payload_types};

// An ordinary Thrift message may own non-Sync state. No Default, Clone,
// Debug, or Protobuf implementation is needed to use the payload contract.
struct SendOnly {
    value: Empty,
    _marker: std::cell::Cell<()>,
}

impl Message for SendOnly {
    fn encode<T: pilota::thrift::TOutputProtocol>(
        &self,
        protocol: &mut T,
    ) -> Result<(), pilota::thrift::ThriftException> {
        self.value.encode(protocol)
    }
    fn decode<T: pilota::thrift::TInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, pilota::thrift::ThriftException> {
        Ok(Self {
            value: Empty::decode(protocol)?,
            _marker: std::cell::Cell::new(()),
        })
    }
    async fn decode_async<T: pilota::thrift::TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, pilota::thrift::ThriftException> {
        Ok(Self {
            value: Empty::decode_async(protocol).await?,
            _marker: std::cell::Cell::new(()),
        })
    }
    fn size<T: pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.value.size(protocol)
    }
}

#[test]
fn ordinary_thrift_message_has_no_default_clone_sync_or_protobuf_requirement() {
    let value = SendOnly {
        value: Empty {},
        _marker: std::cell::Cell::new(()),
    };
    let mut bytes = LinkedBytes::new();
    value
        .encode(&mut TBinaryProtocol::new(&mut bytes, true))
        .unwrap();
    assert_eq!(bytes.concat().as_ref(), [0]);
    let _ = decode::<SendOnly>(&[0]);
}

#[test]
fn arc_service_scalar_void_union_alias_and_recursive_box_data_compile_and_round_trip() {
    use backend_types::rust_types::*;
    let scalar = TypesScalarArgsSend { value: 42 };
    let mut output = LinkedBytes::new();
    scalar
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(decode::<TypesScalarArgsRecv>(&output.concat()).value, 42);
    let mut output = LinkedBytes::new();
    TypesZeroResultSend::Ok(())
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert!(matches!(
        decode::<TypesZeroResultRecv>(&output.concat()),
        TypesZeroResultRecv::Ok(())
    ));
    let message = std::sync::Arc::new(Alias(Event::Value(7)));
    let mut output = LinkedBytes::new();
    message
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(*decode::<std::sync::Arc<Alias>>(&output.concat()), *message);
    let node = Node {
        next: Some(Box::new(Node { next: None })),
    };
    let mut output = LinkedBytes::new();
    node.encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(decode::<Node>(&output.concat()), node);
}

fn decode<M: Message>(bytes: &[u8]) -> M {
    try_decode::<M>(bytes).unwrap()
}

fn try_decode<M: Message>(bytes: &[u8]) -> Result<M, pilota::thrift::ThriftException> {
    let mut bytes = Bytes::copy_from_slice(bytes);
    M::decode(&mut TBinaryProtocol::new(&mut bytes, true))
}

fn encode<M: Message>(message: &M) -> Vec<u8> {
    let size = message.size(&mut TBinaryProtocol::new((), false));
    let mut output = LinkedBytes::new();
    message
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    let bytes = output.concat();
    assert_eq!(size, bytes.len());
    bytes.to_vec()
}

#[test]
fn args_wrap_a_struct_and_results_keep_success_and_exception_field_ids() {
    // Apache Thrift RPC represents arguments as a struct and success as field 0.
    // https://github.com/apache/thrift/blob/master/doc/specs/thrift-rpc.md
    let raw = [11, 0, 1, 0, 0, 0, 1, b'x', 0];
    let message = payload_types::Message {
        payload: Some(Bytes::from_static(b"x")),
        sequence: None,
    };
    assert_eq!(encode(&message), raw);
    assert_eq!(decode::<payload_types::Message>(&raw), message);
    let mut args = vec![12, 0, 1];
    args.extend_from_slice(&raw);
    args.push(0);
    assert_eq!(
        encode(&InteropUnaryArgsSend {
            request: message.clone()
        }),
        args
    );
    assert_eq!(decode::<InteropUnaryArgsRecv>(&args).request, message);
    let mut result = vec![12, 0, 0];
    result.extend_from_slice(&raw);
    result.push(0);
    assert_eq!(encode(&InteropUnaryResultRecv::Ok(message.clone())), result);
    assert!(
        matches!(decode::<InteropUnaryResultRecv>(&result), InteropUnaryResultRecv::Ok(value) if value == message)
    );
    for (value, field_id) in [
        (
            InteropThrowResultRecv::Failure(payload_types::Failure::default()),
            2,
        ),
        (
            InteropThrowResultRecv::Other(payload_types::Other::default()),
            7,
        ),
    ] {
        let bytes = [12, 0, field_id, 0, 0];
        assert_eq!(encode(&value), bytes);
        assert_eq!(decode::<InteropThrowResultRecv>(&bytes), value);
    }
    assert_eq!(encode(&InteropZeroResultSend::Ok(())), [0]);
}

#[test]
fn malformed_results_are_rejected_and_an_empty_struct_consumes_only_stop() {
    assert!(try_decode::<Empty>(&[]).is_err());
    assert!(try_decode::<InteropUnaryResultRecv>(&[0]).is_err());
    // Two declared exception fields cannot form a valid result union.
    assert!(try_decode::<InteropThrowResultRecv>(&[12, 0, 2, 0, 12, 0, 7, 0, 0]).is_err());
    // An unknown exception cannot be mistaken for a successful result.
    assert!(try_decode::<InteropThrowResultRecv>(&[8, 0, 99, 0, 0, 0, 1, 0]).is_err());
    let mut input = Bytes::from_static(&[0, 0xa1, 0xb2]);
    Empty::decode(&mut TBinaryProtocol::new(&mut input, true)).unwrap();
    assert_eq!(input.as_ref(), [0xa1, 0xb2]);
}
