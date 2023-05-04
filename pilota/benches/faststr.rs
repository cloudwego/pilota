#![allow(clippy::redundant_clone)]
use std::{iter::repeat_with, sync::Arc};

use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main};
use faststr::FastStr;
use rand::Rng;

const INLINE_CAP: usize = 22;
const KB: usize = 1024;

fn faststr_bench(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("Bench FastStr Inline");

    let s = gen_string(INLINE_CAP);
    group.bench_function("faststr inline no clone", |b| {
        b.iter(|| {
            black_box(FastStr::new_inline(&s));
        })
    });
    group.bench_function("Arc<str> new no clone", |b| {
        b.iter(|| {
            let _: Arc<str> = black_box(s.as_str().into());
        })
    });

    group.bench_function("faststr inline 1 clone", |b| {
        b.iter(|| {
            let s = FastStr::new_inline(&s);
            let s1 = black_box(s.clone());
            black_box(s1);
        })
    });
    group.bench_function("Arc<str> new 1 clone", |b| {
        b.iter(|| {
            let s: Arc<str> = s.as_str().into();
            let s1 = black_box(s.clone());
            black_box(s1);
        })
    });

    group.bench_function("faststr inline 3 clones", |b| {
        b.iter(|| {
            let s = FastStr::new_inline(&s);
            let s1 = black_box(s.clone());
            let s2 = black_box(s.clone());
            let s3 = black_box(s.clone());
            black_box(s1);
            black_box(s2);
            black_box(s3);
        })
    });
    group.bench_function("Arc<str> new 3 clone", |b| {
        b.iter(|| {
            let s: Arc<str> = s.as_str().into();
            let s1 = black_box(s.clone());
            let s2 = black_box(s.clone());
            let s3 = black_box(s.clone());
            black_box(s1);
            black_box(s2);
            black_box(s3);
        })
    });

    group.finish();

    let mut group = c.benchmark_group("Bench FastStr BytesMut");

    let lens = [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 32 * KB];
    for len in lens {
        let s = gen_string(len);

        #[inline]
        fn gen_bytes_mut(s: &str) -> BytesMut {
            let mut s2 = String::with_capacity(s.len() * 3);
            s2.push_str(s);
            s2.push_str(s);
            s2.push_str(s);
            BytesMut::from(s2.as_str())
        }

        #[inline]
        fn read_faststr_from_bytes_mut(buf: &mut BytesMut) -> FastStr {
            let l = buf.len();
            let b = buf.split_to(l / 3).freeze();
            unsafe { FastStr::from_bytes_unchecked(b) }
        }

        group.bench_function(format!("from bytes no clone, length: {}", len), |b| {
            b.iter_batched_ref(
                || gen_bytes_mut(s.as_str()),
                |buf| {
                    black_box(read_faststr_from_bytes_mut(buf));
                },
                criterion::BatchSize::PerIteration,
            );
        });
        group.bench_function(format!("faststr new no clone, length: {}", len), |b| {
            b.iter(|| {
                black_box(FastStr::new(&s));
            })
        });

        group.bench_function(format!("from bytes 1 clone, length: {}", len), |b| {
            b.iter_batched_ref(
                || gen_bytes_mut(s.as_str()),
                |buf| {
                    let s = read_faststr_from_bytes_mut(buf);
                    let s1 = black_box(s.clone());
                    black_box(s1);
                },
                criterion::BatchSize::PerIteration,
            );
        });
        group.bench_function(format!("faststr new 1 clone, length: {}", len), |b| {
            b.iter(|| {
                let s = FastStr::new(&s);
                let s1 = black_box(s.clone());
                black_box(s1);
            })
        });

        group.bench_function(format!("from bytes 3 clones, length: {}", len), |b| {
            b.iter_batched_ref(
                || gen_bytes_mut(s.as_str()),
                |buf| {
                    let s = read_faststr_from_bytes_mut(buf);
                    let s1 = black_box(s.clone());
                    let s2 = black_box(s.clone());
                    let s3 = black_box(s.clone());
                    black_box(s1);
                    black_box(s2);
                    black_box(s3);
                },
                criterion::BatchSize::PerIteration,
            );
        });
        group.bench_function(format!("faststr new 3 clones, length: {}", len), |b| {
            b.iter(|| {
                let s = FastStr::new(&s);
                let s1 = black_box(s.clone());
                let s2 = black_box(s.clone());
                let s3 = black_box(s.clone());
                black_box(s1);
                black_box(s2);
                black_box(s3);
            })
        });
    }

    group.finish();
}

fn gen_string(size: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    let b: Vec<u8> = repeat_with(|| {
        let i = rng.gen_range(0..CHARSET.len());
        CHARSET[i]
    })
    .take(size)
    .collect();
    let chars: Vec<char> = b.into_iter().map(|b| b as char).collect();
    chars.into_iter().collect()
}

criterion_group!(benches, faststr_bench);
criterion_main!(benches);
