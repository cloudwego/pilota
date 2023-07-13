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
}

union TestUnion {
    1: A a,
    2: B b,
}

typedef list<list<string>> Td

struct D {
    1: required Td td,
}
