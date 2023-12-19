use ahash::{AHashMap, AHashSet};
use bytes::{Bytes, BytesMut};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use faststr::FastStr;
use pilota::thrift::{
    binary_unsafe::TBinaryUnsafeInputProtocol, DecodeError, TInputProtocol, TOutputProtocol,
    TOutputProtocolExt, TStructIdentifier, TType,
};
use rand::Rng;

fn skip_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Thrift Binary Skip Bench");

    let buf = generate_list_i32();
    group.bench_function("binary_unsafe skip list<i32>", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                let b_len = b.len();
                match black_box(skip_binary_unsafe(b, TType::List)) {
                    Ok(size) => assert_eq!(size, b_len),
                    Err(_) => panic!("skip decode error"),
                }
            });
        });
    });

    let buf = generate_struct();
    group.bench_function("binary_unsafe skip struct", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                let b_len = b.len();
                match black_box(skip_binary_unsafe(b, TType::Struct)) {
                    Ok(size) => assert_eq!(size, b_len),
                    Err(_) => panic!("skip decode error"),
                }
            });
        });
    });

    group.finish();
}

fn generate_list_i32() -> Bytes {
    let mut rng = rand::thread_rng();
    let vec_size: usize = 100;
    let v: Vec<i32> = (0..vec_size).map(|_| rng.gen()).collect();

    let mut buf = BytesMut::new();

    let mut p = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    p.write_list(TType::I32, &v, |protocol, e| {
        protocol.write_i32(e.clone())?;
        Ok(())
    })
    .unwrap();
    drop(p);
    buf.freeze()
}

fn generate_struct() -> Bytes {
    let mut buf = BytesMut::new();

    let mut p = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    p.write_struct_begin(&TStructIdentifier { name: "test" })
        .unwrap();
    p.write_i8_field(1, 8).unwrap();
    p.write_i64_field(2, 64).unwrap();
    p.write_uuid_field(3, [1; 16]).unwrap();
    p.write_i32_field(4, 32).unwrap();
    p.write_faststr_field(5, FastStr::new("string")).unwrap();
    p.write_bytes_field(6, Bytes::from("stringbytes")).unwrap();
    p.write_double_field(7, 0.42).unwrap();
    p.write_i16_field(8, 16).unwrap();
    p.write_bool_field(9, false).unwrap();
    p.write_map_field(
        101,
        TType::Binary,
        TType::I32,
        &AHashMap::from([("key1", 1), ("key2", 2), ("key3", 3)]),
        |protocol, key| {
            protocol.write_string(*key)?;
            Ok(())
        },
        |protocol, val| {
            protocol.write_i32(*val)?;
            Ok(())
        },
    )
    .unwrap();
    p.write_set_field(
        201,
        TType::Binary,
        &AHashSet::from(["set1", "set2", "set3"]),
        |protocol, key| {
            protocol.write_string(*key)?;
            Ok(())
        },
    )
    .unwrap();
    p.write_list_field(
        301,
        TType::Binary,
        &["set1", "set2", "set3"],
        |protocol, key| {
            protocol.write_string(*key)?;
            Ok(())
        },
    )
    .unwrap();

    let struct_buf = p.buf_mut().clone();
    p.write_field_begin(TType::Struct, 401).unwrap();
    p.write_bytes_without_len(struct_buf.freeze()).unwrap();
    p.write_field_stop().unwrap();
    p.write_struct_end().unwrap();
    p.write_field_end().unwrap();

    p.write_field_stop().unwrap();
    p.write_struct_end().unwrap();
    drop(p);
    buf.freeze()
}

#[inline(never)]
fn skip_binary_unsafe(mut b: Bytes, ttype: TType) -> Result<usize, DecodeError> {
    unsafe {
        let mut p = TBinaryUnsafeInputProtocol::new(&mut b);
        p.skip_till_depth(ttype, 100)
    }
}

criterion_group!(benches, skip_bench);
criterion_main!(benches);
