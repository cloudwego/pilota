#![allow(clippy::redundant_clone)]

use criterion::{black_box, criterion_group, criterion_main};
use pilota::thrift::{new_protocol_exception, ProtocolExceptionKind, TType, ThriftException};

fn ttype_bench(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("Bench Thrift TType");

    group.bench_function("ttype match 0", |b| {
        b.iter(|| {
            let _ = black_box(OldTType::try_from(0));
        })
    });
    group.bench_function("ttype match 10", |b| {
        b.iter(|| {
            let _ = black_box(OldTType::try_from(10));
        })
    });
    group.bench_function("ttype match 16", |b| {
        b.iter(|| {
            let _ = black_box(OldTType::try_from(16));
        })
    });
    group.bench_function("ttype match 255", |b| {
        b.iter(|| {
            let _ = black_box(OldTType::try_from(255));
        })
    });

    group.bench_function("ttype lookup 0", |b| {
        b.iter(|| {
            let _ = black_box(TType::try_from(0));
        })
    });
    group.bench_function("ttype lookup 10", |b| {
        b.iter(|| {
            let _ = black_box(TType::try_from(10));
        })
    });
    group.bench_function("ttype lookup 16", |b| {
        b.iter(|| {
            let _ = black_box(TType::try_from(16));
        })
    });
    group.bench_function("ttype lookup 255", |b| {
        b.iter(|| {
            let _ = black_box(TType::try_from(255));
        })
    });

    group.finish();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OldTType {
    Stop = 0,
    Void = 1,
    Bool = 2,
    I8 = 3,
    Double = 4,
    I16 = 6,
    I32 = 8,
    I64 = 10,
    Binary = 11,
    Struct = 12,
    Map = 13,
    Set = 14,
    List = 15,
    Uuid = 16,
}

impl TryFrom<u8> for OldTType {
    type Error = ThriftException;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Stop),
            1 => Ok(Self::Void),
            2 => Ok(Self::Bool),
            3 => Ok(Self::I8),
            4 => Ok(Self::Double),
            6 => Ok(Self::I16),
            8 => Ok(Self::I32),
            10 => Ok(Self::I64),
            11 => Ok(Self::Binary),
            12 => Ok(Self::Struct),
            13 => Ok(Self::Map),
            14 => Ok(Self::Set),
            15 => Ok(Self::List),
            16 => Ok(Self::Uuid),
            _ => Err(new_protocol_exception(
                ProtocolExceptionKind::InvalidData,
                format!("invalid ttype {}", value),
            )),
        }
    }
}

criterion_group!(benches, ttype_bench);
criterion_main!(benches);
