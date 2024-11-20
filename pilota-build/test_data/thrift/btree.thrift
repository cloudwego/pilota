struct A {

}

struct B {
    1: required map<i32, list<A>> m(pilota.rust_type = "btree", pilota.rust_wrapper_arc = "true"),
    2: required set<i32> s(pilota.rust_type = "btree"),
    3: required map<list<map<set<i32>, i32>>, set<i32>> m2(pilota.rust_type = "btree"),
}

const map<i32, list<string>> TEST_MAP_LIST = {
    1: ["hello"]
}(pilota.rust_type = "btree")

enum Index {
    A = 0,
    B = 1,
}

const map<Index, string> TEST_MAP = {
    Index.A: "hello",
    Index.B: "world",
}(pilota.rust_type = "btree")

typedef map<set<i32>, string> TypeA(pilota.rust_type = "btree")