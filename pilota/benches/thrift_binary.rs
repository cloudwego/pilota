#![allow(clippy::redundant_clone)]

use bytes::{Bytes, BytesMut};
use criterion::{black_box, criterion_group, criterion_main};
use pilota::thrift::{TInputProtocol, TOutputProtocol};
use rand::{self, Rng};

fn binary_bench(c: &mut criterion::Criterion) {
    let size = std::env::var("SIZE")
        .unwrap_or("10000".to_string())
        .parse()
        .unwrap();
    let mut group = c.benchmark_group("Bench Thrift Binary");
    let mut v: Vec<i64> = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::thread_rng().gen());
    }
    let mut buf = BytesMut::new();

    let mut p = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    for i in &v {
        p.write_i64(*i).unwrap();
    }
    drop(p);
    let buf = buf.freeze();
    assert_eq!(buf.len(), 8 * size);

    let mut buf_le = BytesMut::new();

    let mut p = pilota::thrift::binary_le::TBinaryProtocol::new(&mut buf_le, true);
    for i in &v {
        p.write_i64(*i).unwrap();
    }
    drop(p);
    let buf_le = buf_le.freeze();
    assert_eq!(buf_le.len(), 8 * size);

    let b = buf.clone();
    let v2 = read_be_unsafe_vec(b, size);
    assert_eq!(v, v2);

    group.bench_function("big endian decode vec i64", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                black_box(read_be(b, size));
            });
        })
    });

    group.bench_function("big endian decode vec i64 unsafe", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                black_box(read_be_unsafe(b, size));
            });
        })
    });

    group.bench_function("big endian decode vec i64 unsafe vec", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                black_box(read_be_unsafe_vec(b, size));
            });
        })
    });

    group.bench_function("big endian decode vec i64 unsafe optimized", |b| {
        b.iter(|| {
            black_box({
                let b = buf.clone();
                black_box(read_be_unsafe_optimized(b, size));
            });
        })
    });

    group.bench_function("big endian encode vec i64", |b| {
        b.iter(|| {
            black_box({
                let mut b = BytesMut::with_capacity(8 * size);
                black_box(write_be(&mut b, &v, size));
            });
        })
    });

    group.bench_function("big endian encode vec i64 unsafe", |b| {
        b.iter(|| {
            black_box({
                let mut b = BytesMut::with_capacity(8 * size);
                black_box(write_be_unsafe(&mut b, &v, size));
            });
        })
    });

    group.bench_function("little endian decode vec i64", |b| {
        b.iter(|| {
            black_box({
                let b = buf_le.clone();
                black_box(read_le(b, size));
            });
        })
    });
    group.bench_function("little endian decode vec i64 unsafe optimized", |b| {
        b.iter(|| {
            black_box({
                let b = buf_le.clone();
                black_box(read_le_unsafe_optimized(b, size));
            });
        })
    });
    group.bench_function("little endian decode vec i64 optimized", |b| {
        b.iter(|| {
            black_box({
                let b = buf_le.clone();
                black_box(read_le_optimized(b, size));
            });
        })
    });

    group.bench_function("alloc vec", |b| {
        b.iter(|| {
            let mut b = buf_le.clone();
            let _p = pilota::thrift::binary_le::TBinaryProtocol::new(&mut b, true);
            let _: Vec<i64> = black_box(Vec::with_capacity(size));
        })
    });

    group.finish();
}

#[inline(never)]
fn read_be(mut b: Bytes, size: usize) -> Vec<i64> {
    let mut p = pilota::thrift::binary::TBinaryProtocol::new(&mut b, true);
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(p.read_i64().unwrap());
    }
    v
}

#[inline(never)]
fn read_be_unsafe(mut b: Bytes, size: usize) -> Vec<i64> {
    unsafe {
        let mut p = pilota::thrift::binary_unsafe::TBinaryUnsafeInputProtocol::new(&mut b);
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(p.read_i64().unwrap());
        }
        v
    }
}

#[inline(never)]
fn read_be_unsafe_vec(mut b: Bytes, size: usize) -> Vec<i64> {
    unsafe {
        let mut p = pilota::thrift::binary_unsafe::TBinaryUnsafeInputProtocol::new(&mut b);
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            *v.get_unchecked_mut(i) = p.read_i64().unwrap();
        }
        v.set_len(size);
        v
    }
}

#[inline(never)]
fn read_be_unsafe_optimized(b: Bytes, size: usize) -> Vec<i64> {
    unsafe {
        let buf: &[u8] = b.as_ref();
        assert!(buf.len() >= size * 8);
        let mut index = 0;

        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            *v.get_unchecked_mut(i) = i64::from_be_bytes(
                buf.get_unchecked(index..index + 8)
                    .try_into()
                    .unwrap_unchecked(),
            );
            index += 8;
        }
        v.set_len(size);
        v
    }
}

#[inline(never)]
fn write_be(b: &mut BytesMut, v: &Vec<i64>, _size: usize) {
    let mut p = pilota::thrift::binary::TBinaryProtocol::new(b, true);
    for el in v {
        p.write_i64(*el).unwrap();
    }
}

#[inline(never)]
fn write_be_unsafe(b: &mut BytesMut, v: &Vec<i64>, _size: usize) {
    unsafe {
        let s = std::slice::from_raw_parts_mut(b.as_mut_ptr(), b.len());
        let mut p = pilota::thrift::binary_unsafe::TBinaryUnsafeOutputProtocol::new(b, s, true);
        for el in v {
            p.write_i64(*el).unwrap();
        }
    }
}

#[inline(never)]
fn read_le(mut b: Bytes, size: usize) -> Vec<i64> {
    let mut p = pilota::thrift::binary_le::TBinaryProtocol::new(&mut b, true);

    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(p.read_i64().unwrap());
    }
    v
}

// cargo asm -p pilota --bench thrift_binary --native --full-name --keep-labels
// --simplify --rust
#[inline(never)]
fn read_le_unsafe_optimized(b: Bytes, size: usize) -> Vec<i64> {
    unsafe {
        let buf: &[u8] = b.as_ref();
        assert!(buf.len() >= size * 8);
        let mut index = 0;

        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            *v.get_unchecked_mut(i) = i64::from_le_bytes(
                buf.get_unchecked(index..index + 8)
                    .try_into()
                    .unwrap_unchecked(),
            );
            index += 8;
        }
        v.set_len(size);
        v
    }
}

#[inline(never)]
fn read_le_optimized(mut b: Bytes, size: usize) -> Vec<i64> {
    let _p = pilota::thrift::binary_le::TBinaryProtocol::new(&mut b, true);
    let mut v: Vec<i64> = Vec::with_capacity(size);
    let _ = black_box({
        let src = b.as_ptr();
        let dst = v.as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst as *mut u8, size * 8);
            v.set_len(size);
        }
    });
    v
}

criterion_group!(benches, binary_bench);
criterion_main!(benches);
