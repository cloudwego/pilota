struct A {
    
}

struct B {
    1: required map<i32, list<A>> m(pilota.rust_type = "ahash", pilota.rust_wrapper_arc = "true"),
    2: required set<i32> s(pilota.rust_type = "ahash"),
}

const map<i32, list<string>> TEST_MAP_LIST = {
    1: ["hello"]
}(pilota.rust_type = "ahash")

enum Index {
    A = 0,
    B = 1,
}

const map<Index, string> TEST_MAP = {
    Index.A: "hello",
    Index.B: "world",
}(pilota.rust_type = "ahash")