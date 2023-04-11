enum B {
    Read = 1;
    Write = 2;
}

const string A_S = "string";

struct A {
    1: required string faststr = "hello world",
    2: required string string = "test"(pilota.rust_type = "string"),
    3: optional bool a = false,
    4: optional B test_b = B.Read,
    5: optional B test_b2 = 2,
    6: optional map<string, string> map = {"hello": "world"},
    7: optional double test_double = 1,
    8: optional double test_double2 = 1.2,
    9: optional string alias_str = A_S,
}