use criterion::{Criterion, criterion_group, criterion_main};
use faststr::FastStr;
use pilota::{Bytes, LinkedBytes, pb::Message};
use rand::{Rng, distributions::Alphanumeric};

include!("../test_data/protobuf/normal.new_pb.rs");
include!("../test_data/unknown_fields_pb_new.rs");

fn decode_encode_known_fields_pb(bytes: Bytes) {
    let req = normal::ObjReq::decode(bytes).unwrap();

    let mut out_buf = LinkedBytes::with_capacity(req.encoded_len());
    req.encode(&mut out_buf).unwrap();
}

fn decode_encode_unknown_fields_pb(bytes: Bytes) {
    let req = unknown_fields_pb_new::ObjReq::decode(bytes).unwrap();

    let mut out_buf = LinkedBytes::with_capacity(req.encoded_len());
    req.encode(&mut out_buf).unwrap();
}

fn generate_random_string_pb(size: usize) -> FastStr {
    if size == 0 {
        return FastStr::empty();
    }
    rand::thread_rng()
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

    // size
    let sub_msg_list = vec![sub_msg_1.clone(), sub_msg_2.clone()];

    // size
    let msg_key = normal::Message {
        uid: "".into(),
        value: None,
        sub_messages: vec![sub_msg_1.clone()],
    };

    // size
    let msg_val = normal::SubMessage {
        value: Some(generate_random_string_pb(size)),
    };

    // size * 2
    let msg_map_entry = normal::obj_req::MsgMapEntry {
        key: Some(msg_key),
        value: Some(msg_val),
    };

    // size * 2
    let msg_for_set_and_field = normal::Message {
        uid: "".into(),
        value: Some(generate_random_string_pb(size)),
        sub_messages: sub_msg_list.clone(),
    };

    // size * 2
    let mut sub_msg_list2 = vec![sub_msg_1, sub_msg_2];
    sub_msg_list2.extend(sub_msg_list.clone());

    normal::ObjReq {
        msg: Some(msg_for_set_and_field.clone()), // size * 2
        msg_map: vec![msg_map_entry],             // size * 2
        sub_msgs: sub_msg_list2,                  // size * 2
        msg_set: vec![msg_for_set_and_field],     // size * 2
        flag_msg: "".into(),
        mock_cost: None,
    }
}

fn pb_codegen(c: &mut Criterion) {
    let mut group = c.benchmark_group("Protobuf_New Bench Unknown Fields");
    let lens = [16, 64, 128, 512, 2 * 1024, 128 * 1024, 10 * 128 * 1024];
    for len_param in lens {
        let req_instance = prepare_obj_req_pb(len_param);
        let mut encoded_known_bytes_lb = LinkedBytes::with_capacity(req_instance.encoded_len());
        req_instance.encode(&mut encoded_known_bytes_lb).unwrap();
        let encoded_known_bytes = encoded_known_bytes_lb.bytes().clone().freeze();

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
