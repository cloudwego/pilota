enum Index {
    A = 0x01,
    B = 0x10,
}

enum Err {

}

enum Ok {
}

enum _Enum {
    _1 = 1,
    _2 = 2
}

struct Request {
    1: required Index Index,
    2: Index index,
    3: _Enum _enum,
}
service Test {
   Err test_enum(1: Ok req);
   Err test_enum_var_type_name_conflict (1: Request req);
}