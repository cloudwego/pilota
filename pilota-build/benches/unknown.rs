use criterion::{Criterion, criterion_group, criterion_main};
use faststr::FastStr;
use pilota::{
    Bytes,
    prost::bytes::BytesMut,
    thrift::{
        Message,
        binary::TBinaryProtocol,
        binary_unsafe::{TBinaryUnsafeInputProtocol, TBinaryUnsafeOutputProtocol},
    },
};
use rand::{Rng, distributions::Alphanumeric};

include!("../test_data/thrift/normal.rs");
include!("../test_data/unknown_fields.rs");

fn decode_encode_all_fields_safe(mut bytes: Bytes) {
    let a =
        crate::normal::normal::ObjReq::decode(&mut TBinaryProtocol::new(&mut bytes, true)).unwrap();

    let size = a.size(&mut TBinaryProtocol::new((), false));
    let mut linked_bytes = linkedbytes::LinkedBytes::with_capacity(size);
    a.encode(&mut TBinaryProtocol::new(&mut linked_bytes, true))
        .unwrap();
}

fn decode_encode_all_fields_unsafe(mut bytes: Bytes) {
    let a = unsafe {
        crate::normal::normal::ObjReq::decode(&mut TBinaryUnsafeInputProtocol::new(&mut bytes))
    }
    .unwrap();

    let size = a.size(&mut TBinaryProtocol::new((), false));
    let mut linked_bytes = linkedbytes::LinkedBytes::with_capacity(size);
    let buf = unsafe {
        let l = linked_bytes.bytes_mut().len();
        std::slice::from_raw_parts_mut(
            linked_bytes.bytes_mut().as_mut_ptr().add(l),
            linked_bytes.bytes_mut().capacity() - l,
        )
    };
    unsafe {
        a.encode(&mut TBinaryUnsafeOutputProtocol::new(
            &mut linked_bytes,
            buf,
            true,
        ))
        .unwrap();
    }
}

fn decode_encode_unknown_fields_safe(mut bytes: Bytes) {
    let a = crate::unknown_fields::unknown_fields::ObjReq::decode(&mut TBinaryProtocol::new(
        &mut bytes, true,
    ))
    .unwrap();

    let size = a.size(&mut TBinaryProtocol::new((), false));
    let mut linked_bytes = linkedbytes::LinkedBytes::with_capacity(size);
    a.encode(&mut TBinaryProtocol::new(&mut linked_bytes, true))
        .unwrap();
}

fn decode_encode_unknown_fields_unsafe(mut bytes: Bytes) {
    let a = unsafe {
        crate::unknown_fields::unknown_fields::ObjReq::decode(&mut TBinaryUnsafeInputProtocol::new(
            &mut bytes,
        ))
    }
    .unwrap();

    let size = a.size(&mut TBinaryProtocol::new((), false));
    let mut linked_bytes = linkedbytes::LinkedBytes::with_capacity(size);
    let buf = unsafe {
        let l = linked_bytes.bytes_mut().len();
        std::slice::from_raw_parts_mut(
            linked_bytes.bytes_mut().as_mut_ptr().add(l),
            linked_bytes.bytes_mut().capacity() - l,
        )
    };
    unsafe {
        a.encode(&mut TBinaryUnsafeOutputProtocol::new(
            &mut linked_bytes,
            buf,
            true,
        ))
        .unwrap();
    }
}

fn codegen(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bench Unknown Fields");
    let lens = [16, 64, 128, 512, 2 * 1024, 128 * 1024, 10 * 128 * 1024];
    for len in lens {
        let a = prepare_obj_req(len);
        let size = a.size(&mut TBinaryProtocol::new((), false));
        let mut buf = BytesMut::with_capacity(size);
        a.encode(&mut TBinaryProtocol::new(&mut buf, false))
            .unwrap();
        let buf = buf.freeze();
        group.bench_function(
            format!("TBinaryProtocol all_fields decode_encode {} bytes", len * 8),
            |b| b.iter_with_setup(|| buf.clone(), decode_encode_all_fields_safe),
        );
        group.bench_function(
            format!(
                "TBinaryUnsafeProtocol all_fields
  decode_encode {} bytes",
                len * 8
            ),
            |b| b.iter_with_setup(|| buf.clone(), decode_encode_all_fields_unsafe),
        );
        group.bench_function(
            format!(
                "TBinaryProtocol unknown_fields decode_encode {} bytes",
                len * 8
            ),
            |b| b.iter_with_setup(|| buf.clone(), decode_encode_unknown_fields_safe),
        );
        group.bench_function(
            format!(
                "TBinaryUnsafeProtocol unknown_fields
  decode_encode {}
bytes",
                len * 8
            ),
            |b| b.iter_with_setup(|| buf.clone(), decode_encode_unknown_fields_unsafe),
        );
    }
}

criterion_group!(benches, codegen);
criterion_main!(benches);

fn prepare_obj_req(size: usize) -> crate::normal::normal::ObjReq {
    let mut req = crate::normal::normal::ObjReq::default();
    let sub_msg_1 = crate::normal::normal::SubMessage {
        value: Some(generate_message(size / 2)),
    };
    let sub_msg_2 = crate::normal::normal::SubMessage {
        value: Some(generate_message(size / 2)),
    };
    // size
    let sub_msg_list = vec![sub_msg_1.clone(), sub_msg_2.clone()];

    // 2 * size
    let msg = crate::normal::normal::Message {
        value: Some(generate_message(size)),
        sub_messages: Some(sub_msg_list.clone()),
        uid: None,
    };

    let mut msg_map = pilota::AHashMap::default();
    msg_map.insert(
        crate::normal::normal::Message {
            value: None,
            sub_messages: Some(sub_msg_list.clone()),
            uid: None,
        },
        crate::normal::normal::SubMessage {
            value: Some(generate_message(size)),
        },
    );

    let mut msg_set = pilota::AHashSet::default();
    msg_set.insert(msg.clone());

    let mut sub_msg_list2 = vec![sub_msg_1, sub_msg_2];
    sub_msg_list2.extend(sub_msg_list);

    req.msg = msg; // 2 * size
    req.msg_map = msg_map; // 2 * size
    req.msg_set = Some(msg_set); // 2 * size
    req.sub_msgs = sub_msg_list2; // 2 * size

    req
}

fn generate_message(size: usize) -> FastStr {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
