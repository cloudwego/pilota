include "must_gen_items.thrift"
include "thrift/void.thrift"

struct A {
    1: required binary bytes,
    2: required binary vec(pilota.rust_type="vec"),
}

struct B {
    1: required string faststr,
    2: required string string(pilota.rust_type = "string"),
    3: required list<list<string>> list,
}

struct C {
    1: required must_gen_items.A a,
}

enum Index {
    A = 0,
    B = 1,
}

const map<Index, string> TEST_MAP = {
    Index.A: "hello",
    Index.B: "world",
};


const list<string> TEST_LIST = [
    "hello",
    "world",
];


const map<i32, list<string>> TEST_MAP_LIST = {
    1: ["hello"]
}

service Test {
   void test_123();
   ObjReq testException(1: ObjReq req) throws (1: STException stException);
}

exception STException {
    1: string message;
}

union TestUnion {
    1: A a,
    2: B b,
}

typedef list<list<string>> Td

struct D {
    1: required Td td,
}

struct SubMessage {

}

struct Message {

}

struct ObjReq {
    
}
