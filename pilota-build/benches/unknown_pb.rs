use criterion::{Criterion, criterion_group, criterion_main};
use faststr::FastStr;
use pilota::{Bytes, LinkedBytes, prost::Message};
use rand::{Rng, distr::Alphanumeric};

include!("../test_data/protobuf/normal.rs");
include!("../test_data/unknown_fields_pb.rs");

fn decode_encode_known_fields_pb(bytes: Bytes) {
    let req = normal::ObjReq::decode(bytes).unwrap();

    let mut out_buf = LinkedBytes::with_capacity(req.encoded_len());
    req.encode(&mut out_buf).unwrap();
}

fn decode_encode_unknown_fields_pb(bytes: Bytes) {
    let req = unknown_fields_pb::ObjReq::decode(bytes).unwrap();

    let mut out_buf = LinkedBytes::with_capacity(req.encoded_len());
    req.encode(&mut out_buf).unwrap();
}

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

fn prepare_obj_req_pb(size: usize) -> normal::ObjReq {
    let sub_msg_1 = normal::SubMessage {
        value: Some(generate_random_string_pb(size / 2)),
    };
    let sub_msg_2 = normal::SubMessage {
        value: Some(generate_random_string_pb(size / 2)),
    };
    let sub_msg_list = vec![sub_msg_1.clone(), sub_msg_2.clone()];

    let msg = normal::Message {
        uid: "".into(),
        value: Some(generate_random_string_pb(size)),
        sub_messages: sub_msg_list.clone(),
    };

    let msg_map_key = normal::Message {
        uid: "".into(),
        value: None,
        sub_messages: sub_msg_list.clone(),
    };

    let msg_map_val = normal::SubMessage {
        value: Some(generate_random_string_pb(size)),
    };

    let msg_map_entry = normal::obj_req::MsgMapEntry {
        key: Some(msg_map_key),
        value: Some(msg_map_val),
    };
    let mut sub_msg_list2 = vec![sub_msg_1.clone(), sub_msg_2.clone()];
    sub_msg_list2.extend(sub_msg_list.clone());

    normal::ObjReq {
        msg: Some(msg.clone()),       // 2 * size
        msg_map: vec![msg_map_entry], // 2 * size
        sub_msgs: sub_msg_list2,      // 2 * size
        msg_set: vec![msg],           // 2 * size
        flag_msg: "".into(),
        mock_cost: None,
    }
}

fn pb_codegen(c: &mut Criterion) {
    let mut group = c.benchmark_group("Protobuf Bench Unknown Fields");
    let lens = [16, 64, 128, 512, 2 * 1024, 128 * 1024, 10 * 128 * 1024];
    for len_param in lens {
        // Prepare data using the "normal" struct definition
        let req_instance = prepare_obj_req_pb(len_param);
        let mut encoded_known_bytes = LinkedBytes::with_capacity(req_instance.encoded_len());
        req_instance.encode(&mut encoded_known_bytes).unwrap();
        let encoded_known_bytes = encoded_known_bytes.bytes().clone().freeze();

        group.bench_function(
            format!("PB KnownFields DecodeEncode {} bytes", len_param * 8),
            |b| {
                b.iter_with_setup(
                    || encoded_known_bytes.clone(), // Clone immutable Bytes (cheap)
                    decode_encode_known_fields_pb,
                )
            },
        );

        group.bench_function(
            format!("PB UnknownFields DecodeEncode {} bytes", len_param * 8),
            |b| {
                b.iter_with_setup(
                    || encoded_known_bytes.clone(), // Clone immutable Bytes (cheap)
                    decode_encode_unknown_fields_pb,
                )
            },
        );
    }
}

criterion_group!(pb_benches, pb_codegen);
criterion_main!(pb_benches);
