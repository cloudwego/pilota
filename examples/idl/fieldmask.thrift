namespace rs fieldmask

include "base.thrift"

struct A {
    1: i32 a,
    2: string b,
}

struct Request {
    1: bool f1,
    2: i8 f2,
    3: i16 f3,
    4: i32 f4,
    5: i64 f5,
    6: double f6,
    7: string f7,
    8: binary f8,
    9: required list<i32> f9,
    10: set<string> f10,
    11: A f11,
    12: list<list<i32>> f12,
    13: list<A> f13,
    14: map<i32, string> f14,
    15: map<string, A> f15,
    16: map<string, list<A>> f16,
    17: list<map<string, i32>> f17,
    255: optional base.Base base
}

struct Response {}

service Test {
    Response test(1: Request req);
} 