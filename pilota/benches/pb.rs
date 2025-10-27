#![allow(clippy::redundant_clone)]

use std::hint::black_box;

use bytes::Bytes;
use criterion::{BenchmarkId, Throughput, criterion_group, criterion_main};
use faststr::FastStr;
use linkedbytes::LinkedBytes;
use pilota::pb::{Message, encoding::EncodeLengthContext};
use rand::{Rng, distr::Alphanumeric};

include!("../../pilota-build/test_data/protobuf/normal.rs");

const KB: usize = 1024;
const SIZES: &[usize] = &[16, 64, 128, 512, 2 * KB, 128 * KB, 10 * 128 * KB];

#[inline]
fn varint_len(mut n: usize) -> usize {
    let mut len = 1;
    while n >= 0x80 {
        n >>= 7;
        len += 1;
    }
    len
}

#[inline]
fn payload_len_for_total(total_size: usize) -> usize {
    // total = tag_len(1) + varint(payload_len) + payload_len
    // Try varint length from 1..=5 to find consistent payload_len.
    for vlen in 1..=5 {
        if total_size < 1 + vlen {
            continue;
        }
        let payload_len = total_size - 1 - vlen;
        if varint_len(payload_len) == vlen {
            return payload_len;
        }
    }
    panic!("cannot construct payload for total_size={}", total_size);
}

#[inline]
fn make_payload(total_size: usize) -> Vec<u8> {
    let len = payload_len_for_total(total_size);
    let mut v = Vec::with_capacity(len);
    // Deterministic but non-trivial content
    for i in 0..len {
        v.push(((i as u32).wrapping_mul(1315423911) as u8) ^ 0x5A);
    }
    v
}

fn pb_bench(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("PB Message Vec<u8> (BytesValue)");

    for &total in SIZES.iter() {
        let msg: Vec<u8> = make_payload(total);

        // Pre-compute capacities
        let mut ctx0 = EncodeLengthContext::default();
        let required_no_ld = msg.encoded_len(&mut ctx0);
        let mut ctx1 = EncodeLengthContext::default();
        let (_len_only, required_ld_total) = msg.encoded_len_length_delimited(&mut ctx1);

        // Pre-encode for decode benches
        let mut ctx_enc = EncodeLengthContext::default();
        let enc = msg.encode_to_vec(&mut ctx_enc);
        let bytes_normal = Bytes::from(enc);

        let mut ctx_enc_ld = EncodeLengthContext::default();
        let enc_ld = msg.encode_length_delimited_to_vec(&mut ctx_enc_ld);
        let bytes_ld = Bytes::from(enc_ld);

        group.throughput(Throughput::Bytes(total as u64));

        group.bench_function(BenchmarkId::new("encode", total), |b| {
            b.iter(|| {
                let mut buf = LinkedBytes::with_capacity(required_no_ld);
                msg.encode(&mut buf).unwrap();
                black_box(buf);
            })
        });

        group.bench_function(BenchmarkId::new("encode_to_vec", total), |b| {
            b.iter(|| {
                let mut ctx = EncodeLengthContext::default();
                let v = msg.encode_to_vec(&mut ctx);
                black_box(v);
            })
        });

        group.bench_function(BenchmarkId::new("encode_length_delimited", total), |b| {
            b.iter(|| {
                let mut ctx = EncodeLengthContext::default();
                let mut buf = LinkedBytes::with_capacity(required_ld_total);
                msg.encode_length_delimited(&mut ctx, &mut buf).unwrap();
                black_box(buf);
            })
        });

        group.bench_function(
            BenchmarkId::new("encode_length_delimited_to_vec", total),
            |b| {
                b.iter(|| {
                    let mut ctx = EncodeLengthContext::default();
                    let v = msg.encode_length_delimited_to_vec(&mut ctx);
                    black_box(v);
                })
            },
        );

        group.bench_function(BenchmarkId::new("decode", total), |b| {
            b.iter(|| {
                let decoded = <Vec<u8> as Message>::decode(bytes_normal.clone()).unwrap();
                black_box(decoded);
            })
        });

        group.bench_function(BenchmarkId::new("decode_length_delimited", total), |b| {
            b.iter(|| {
                let decoded =
                    <Vec<u8> as Message>::decode_length_delimited(bytes_ld.clone()).unwrap();
                black_box(decoded);
            })
        });
    }

    group.finish();
}

#[inline]
fn generate_random_string_pb(size: usize) -> FastStr {
    if size == 0 {
        return FastStr::empty();
    }
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect::<String>()
        .into()
}

#[inline]
fn prepare_obj_req_pb(size: usize) -> normal::ObjReq {
    let sub_msg_1 = normal::SubMessage {
        value: Some(generate_random_string_pb(size / 2)),
    };
    let sub_msg_2 = normal::SubMessage {
        value: Some(generate_random_string_pb(size / 2)),
    };

    let sub_msg_list = vec![sub_msg_1.clone(), sub_msg_2.clone()];

    let msg_key = normal::Message {
        uid: "".into(),
        value: Some(generate_random_string_pb(size / 2)),
        sub_messages: vec![sub_msg_1.clone()],
    };

    let msg_val = normal::SubMessage {
        value: Some(generate_random_string_pb(size)),
    };

    let msg_map_entry = normal::obj_req::MsgMapEntry {
        key: Some(msg_key),
        value: Some(msg_val),
    };

    let msg_for_set_and_field = normal::Message {
        uid: "".into(),
        value: Some(generate_random_string_pb(size)),
        sub_messages: sub_msg_list.clone(),
    };

    let mut sub_msg_list2 = vec![sub_msg_1, sub_msg_2];
    sub_msg_list2.extend(sub_msg_list.clone());

    normal::ObjReq {
        msg: Some(msg_for_set_and_field.clone()),
        msg_map: vec![msg_map_entry],
        sub_msgs: sub_msg_list2,
        msg_set: vec![msg_for_set_and_field],
        flag_msg: "".into(),
        mock_cost: None,
    }
}

fn pb_bench_normal(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("PB Message normal::ObjReq");

    for &size_param in SIZES.iter() {
        let msg = prepare_obj_req_pb(size_param);

        // 计算真实编码长度，用于容量与吞吐统计
        let mut ctx0 = EncodeLengthContext::default();
        let required_no_ld = msg.encoded_len(&mut ctx0);
        let mut ctx1 = EncodeLengthContext::default();
        let (_len_only, required_ld_total) = msg.encoded_len_length_delimited(&mut ctx1);

        // 预编码为 Bytes 用于解码基准
        let mut ctx_enc = EncodeLengthContext::default();
        let enc = msg.encode_to_vec(&mut ctx_enc);
        let bytes_normal = Bytes::from(enc);

        let mut ctx_enc_ld = EncodeLengthContext::default();
        let enc_ld = msg.encode_length_delimited_to_vec(&mut ctx_enc_ld);
        let bytes_ld = Bytes::from(enc_ld);

        group.throughput(Throughput::Bytes(required_no_ld as u64));

        group.bench_function(BenchmarkId::new("encode", size_param), |b| {
            b.iter(|| {
                let mut buf = LinkedBytes::with_capacity(required_no_ld);
                msg.encode(&mut buf).unwrap();
                black_box(buf);
            })
        });

        group.bench_function(BenchmarkId::new("encode_to_vec", size_param), |b| {
            b.iter(|| {
                let mut ctx = EncodeLengthContext::default();
                let v = msg.encode_to_vec(&mut ctx);
                black_box(v);
            })
        });

        group.bench_function(
            BenchmarkId::new("encode_length_delimited", size_param),
            |b| {
                b.iter(|| {
                    let mut ctx = EncodeLengthContext::default();
                    let mut buf = LinkedBytes::with_capacity(required_ld_total);
                    msg.encode_length_delimited(&mut ctx, &mut buf).unwrap();
                    black_box(buf);
                })
            },
        );

        group.bench_function(
            BenchmarkId::new("encode_length_delimited_to_vec", size_param),
            |b| {
                b.iter(|| {
                    let mut ctx = EncodeLengthContext::default();
                    let v = msg.encode_length_delimited_to_vec(&mut ctx);
                    black_box(v);
                })
            },
        );

        group.bench_function(BenchmarkId::new("decode", size_param), |b| {
            b.iter(|| {
                let decoded = <normal::ObjReq as Message>::decode(bytes_normal.clone()).unwrap();
                black_box(decoded);
            })
        });

        group.bench_function(
            BenchmarkId::new("decode_length_delimited", size_param),
            |b| {
                b.iter(|| {
                    let decoded =
                        <normal::ObjReq as Message>::decode_length_delimited(bytes_ld.clone())
                            .unwrap();
                    black_box(decoded);
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, pb_bench, pb_bench_normal);
criterion_main!(benches);
