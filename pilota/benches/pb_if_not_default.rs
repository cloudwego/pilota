#![allow(clippy::redundant_clone)]

use std::hint::black_box;

use criterion::{BenchmarkId, Throughput, criterion_group, criterion_main};
use linkedbytes::LinkedBytes;
use pilota::pb::{encoding, encoding::EncodeLengthContext};
use rand::Rng;

fn bench_int32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("pb_if_not_default::int32");

    // Non-zero value path: encode_if_not_default vs encode
    let v_nonzero: i32 = 12345;
    let mut ctx = EncodeLengthContext::default();
    let need = encoding::int32::encoded_len(&mut ctx, 1, &v_nonzero);
    group.throughput(Throughput::Bytes(need as u64));

    group.bench_function(BenchmarkId::new("nonzero/if_not_default", 1), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need);
            encoding::int32::encode_if_not_default(1, &v_nonzero, &mut buf);
            black_box(buf);
        })
    });
    group.bench_function(BenchmarkId::new("nonzero/encode", 1), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need);
            encoding::int32::encode(1, &v_nonzero, &mut buf);
            black_box(buf);
        })
    });

    // Zero value path: if_not_default (skip) vs encode (write 0)
    let v_zero: i32 = 0;
    let mut ctx0 = EncodeLengthContext::default();
    let need0 = encoding::int32::encoded_len(&mut ctx0, 1, &v_zero);

    group.throughput(Throughput::Bytes(need0 as u64));
    group.bench_function(BenchmarkId::new("zero/if_not_default(skip)", 1), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need0);
            encoding::int32::encode_if_not_default(1, &v_zero, &mut buf);
            black_box(buf);
        })
    });
    group.bench_function(BenchmarkId::new("zero/encode(write 0)", 1), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need0);
            encoding::int32::encode(1, &v_zero, &mut buf);
            black_box(buf);
        })
    });

    group.finish();
}

fn bench_string(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("pb_if_not_default::string");

    // Non-empty string path
    let s = "hello world";
    let mut ctx = EncodeLengthContext::default();
    let need = encoding::string::encoded_len(&mut ctx, 1, &s);
    group.throughput(Throughput::Bytes(need as u64));

    group.bench_function(BenchmarkId::new("nonempty/if_not_default", s.len()), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need);
            encoding::string::encode_if_not_default(1, &s, &mut buf);
            black_box(buf);
        })
    });
    group.bench_function(BenchmarkId::new("nonempty/encode", s.len()), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need);
            encoding::string::encode(1, &s, &mut buf);
            black_box(buf);
        })
    });

    // Empty string path
    let s_empty = "";
    let mut ctx0 = EncodeLengthContext::default();
    let need0 = encoding::string::encoded_len(&mut ctx0, 1, &s_empty);

    group.bench_function(BenchmarkId::new("empty/if_not_default(skip)", 0), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need0);
            encoding::string::encode_if_not_default(1, &s_empty, &mut buf);
            black_box(buf);
        })
    });
    group.bench_function(BenchmarkId::new("empty/encode(write \"\")", 0), |b| {
        b.iter(|| {
            let mut buf = LinkedBytes::with_capacity(need0);
            encoding::string::encode(1, &s_empty, &mut buf);
            black_box(buf);
        })
    });

    group.finish();
}

fn bench_int32_random(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("pb_if_not_default::int32_random");

    // A batch of values ​​is pre-generated: 50% are 0, and 50% are random non-zero
    // values, in order to break the branch prediction as much as possible.
    let n = 4096;
    let mut rng = rand::rng();
    let vals: Vec<i32> = (0..n)
        .map(|i| {
            if i & 1 == 0 {
                0
            } else {
                rng.random_range(1..=1_000_000)
            }
        })
        .collect();

    // Random access, cyclical value selection (avoiding the introduction of
    // additional random overhead in measurement).
    let mut idx = 0usize;
    group.bench_function(BenchmarkId::new("random/if_not_default", n), |b| {
        b.iter(|| {
            let v = vals[idx & (n - 1)];
            idx = idx.wrapping_add(1);
            let mut buf = LinkedBytes::with_capacity(16);
            encoding::int32::encode_if_not_default(1, &v, &mut buf);
            black_box(buf);
        })
    });

    let mut idx2 = 0usize;
    group.bench_function(BenchmarkId::new("random/encode", n), |b| {
        b.iter(|| {
            let v = vals[idx2 & (n - 1)];
            idx2 = idx2.wrapping_add(1);
            let mut buf = LinkedBytes::with_capacity(16);
            encoding::int32::encode(1, &v, &mut buf);
            black_box(buf);
        })
    });

    group.finish();
}

fn bench_string_random(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("pb_if_not_default::string_random");

    // Pre-generated strings: 50% empty strings, 50% random length and content
    let n = 4096;
    let mut rng = rand::rng();
    let vals: Vec<String> = (0..n)
        .map(|i| {
            if i & 1 == 0 {
                String::new()
            } else {
                let len = rng.random_range(1..=32usize);
                let mut s = String::with_capacity(len);
                for _ in 0..len {
                    // 简单 ASCII
                    let ch = rng.random_range(b'a'..=b'z') as char;
                    s.push(ch);
                }
                s
            }
        })
        .collect();

    let mut idx = 0usize;
    group.bench_function(BenchmarkId::new("random/if_not_default", n), |b| {
        b.iter(|| {
            let v = &vals[idx & (n - 1)];
            idx = idx.wrapping_add(1);
            let mut buf = LinkedBytes::with_capacity(64);
            encoding::string::encode_if_not_default(1, v, &mut buf);
            black_box(buf);
        })
    });

    let mut idx2 = 0usize;
    group.bench_function(BenchmarkId::new("random/encode", n), |b| {
        b.iter(|| {
            let v = &vals[idx2 & (n - 1)];
            idx2 = idx2.wrapping_add(1);
            let mut buf = LinkedBytes::with_capacity(64);
            encoding::string::encode(1, v, &mut buf);
            black_box(buf);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_int32,
    bench_string,
    bench_int32_random,
    bench_string_random,
    bench_repeated_int32
);
criterion_main!(benches);

fn bench_repeated_int32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("pb_if_not_default::repeated_int32");
    // Test slices of different lengths (arrays/vectors can be passed as slices).
    let sizes = [0usize, 1, 16, 64, 256, 1024, 8192];
    for &n in &sizes {
        // Construct the data: half of it is 0, and the other half is random non-zero
        // (breaking the data distribution).
        let mut rng = rand::rng();
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            if i & 1 == 0 {
                v.push(0i32);
            } else {
                v.push(rng.random_range(1..=1_000_000));
            }
        }

        // Calculate length to set throughput
        let len_repeated =
            encoding::int32::encoded_len_repeated(&mut EncodeLengthContext::default(), 1, &v);
        let len_packed = encoding::int32::encoded_len_packed(1, &v);

        group.throughput(Throughput::Bytes(len_repeated.max(len_packed) as u64));

        group.bench_function(BenchmarkId::new("repeated/encode_repeated", n), |b| {
            b.iter(|| {
                let mut buf = LinkedBytes::with_capacity(len_repeated);
                encoding::int32::encode_repeated(1, &v, &mut buf);
                black_box(buf);
            })
        });

        group.bench_function(BenchmarkId::new("repeated/encode_packed", n), |b| {
            b.iter(|| {
                let mut buf = LinkedBytes::with_capacity(len_packed);
                encoding::int32::encode_packed(1, &v, &mut buf);
                black_box(buf);
            })
        });
    }
    group.finish();
}
