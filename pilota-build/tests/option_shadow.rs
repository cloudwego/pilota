//! IDL types named `SOME`, `NONE` and `OPTION` are emitted as Rust structs
//! `Some`, `None` and `Option`, which shadow the prelude names inside the
//! generated module. The generated code must still compile and work.

use pilota::{
    Bytes, LinkedBytes,
    pb::EncodeLengthContext,
    thrift::{Message, binary::TBinaryProtocol},
};

mod thrift {
    include!("../test_data/thrift/option_shadow.rs");
}

mod pb {
    include!("../test_data/protobuf/option_shadow.rs");
}

use pb::option_shadow::option_shadow as p;
use thrift::option_shadow::option_shadow as t;

fn thrift_round_trip<M: Message>(msg: &M) -> M {
    let mut size_protocol = TBinaryProtocol::new((), false);
    let size = msg.size(&mut size_protocol);
    let mut buf = LinkedBytes::with_capacity(size);
    msg.encode(&mut TBinaryProtocol::new(&mut buf, false))
        .unwrap();
    let mut bytes = buf.concat().freeze();
    M::decode(&mut TBinaryProtocol::new(&mut bytes, false)).unwrap()
}

fn pb_round_trip<M: pilota::pb::Message + Default>(msg: &M) -> M {
    let bytes = msg.encode_to_vec(&mut EncodeLengthContext::default());
    M::decode(Bytes::from(bytes)).unwrap()
}

#[test]
fn thrift_enum_try_from_uses_prelude_option() {
    assert_eq!(t::Some::try_from_i32(1), Some(t::Some::ONE));
    assert_eq!(t::Some::try_from_i32(2), None);
    assert_eq!(t::None::try_from_i32(0), Some(t::None::ZERO));
    assert_eq!(t::Option::try_from_i32(0), Some(t::Option::ANY));
}

#[test]
fn thrift_default_values() {
    let item = t::Item::default();
    assert_eq!(item.with_default.as_deref(), Some("default"));
    assert_eq!(item.lazy_default, Some(vec![1, 2]));
    assert_eq!(item.name, None);

    assert_eq!(t::DEFAULT_ITEM.name.as_deref(), Some("const"));
    assert_eq!(t::DEFAULT_ITEM.nums, None);
}

#[test]
fn thrift_struct_and_union_round_trip() {
    let item = t::Item {
        name: Some("item".into()),
        some_code: t::Some::ONE,
        none_code: t::None::ZERO,
        option_code: Some(t::Option::ANY),
        nums: Some(vec![1, 2, 3]),
        tags: Some([("k".into(), "v".into())].into_iter().collect()),
        ids: Some([7].into_iter().collect()),
        inner: Some(t::Inner { id: 42 }),
        with_default: Some("default".into()),
        lazy_default: Some(vec![1, 2]),
    };
    assert_eq!(thrift_round_trip(&item), item);

    let choice = t::Choice::Item(item);
    assert_eq!(thrift_round_trip(&choice), choice);
}

#[test]
fn pb_enum_try_from_uses_prelude_option() {
    assert_eq!(p::Some::try_from_i32(1), Some(p::Some::SOME_ONE));
    assert_eq!(p::Some::try_from_i32(2), None);
    assert_eq!(p::None::try_from_i32(0), Some(p::None::NONE_ZERO));
}

#[test]
fn pb_message_round_trip() {
    let item = p::Item {
        name: Some("item".into()),
        some_code: p::Some::SOME_ONE,
        none_code: p::None::NONE_ZERO,
        nums: vec![1, 2, 3],
        tags: [("k".into(), "v".into())].into_iter().collect(),
        inner: Some(p::Inner { id: 42 }),
        choice: Some(p::item::Choice::Other(p::Inner { id: 7 })),
    };
    assert_eq!(pb_round_trip(&item), item);
}
