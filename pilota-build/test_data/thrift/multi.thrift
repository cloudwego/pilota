include "default_value.thrift"
include "recursive_type.thrift"

struct A {
    1: default_value.C c = {"off": "off"},
}

struct B {
    1: recursive_type.A a,
    2: recursive_type.C c, 
}
