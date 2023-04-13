use criterion::{criterion_group, criterion_main, Criterion};
use pilota::{
    prost::bytes::BytesMut,
    thrift::{binary::TBinaryProtocol, Message},
};

include!("../test_data/thrift/default_value.rs");

fn decode(bytes: &[u8]) {
    let _a = default_value::default_value::A::decode(
        &mut pilota::thrift::binary::TBinaryProtocol::new(&mut BytesMut::from(bytes), false),
    )
    .unwrap();
}

fn codegen(c: &mut Criterion) {
    let a = default_value::default_value::A::default();
    let size = a.size(&mut TBinaryProtocol::new((), false));
    let mut buf = BytesMut::with_capacity(size);
    a.encode(&mut TBinaryProtocol::new(&mut buf, false))
        .unwrap();
    c.bench_function("decode", |b| b.iter(|| decode(&buf)));
}

criterion_group!(benches, codegen);
criterion_main!(benches);
