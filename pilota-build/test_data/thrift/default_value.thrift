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
    6: optional i8 test_b3 = B.Read,
    7: optional map<string, string> map = {"hello": "world"},
    8: optional double test_double = 1,
    9: optional double test_double2 = 1.2,
    10: optional string alias_str = A_S,
    11: required binary empty = "",
    12: required map<double, double> test_map = {1.0: 2.0},
    13: required set<double> test_set = [1.0],
    14: bool a2 = 3,
    15: map<string, string> map2 = [],
 }

struct C {
    1: string off = "off",
    2: optional byte test_byte = 0,
}