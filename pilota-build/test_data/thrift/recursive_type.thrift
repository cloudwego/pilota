struct A {
    1: optional A a,
    2: optional B a_b,
}

struct B {
    1: optional A b_a,
}

struct C {
    1: set<string> c,
}
