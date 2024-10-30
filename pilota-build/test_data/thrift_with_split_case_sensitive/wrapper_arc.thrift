struct A {

}

struct TEST {
    1: required string ID,
    2: required list<list<A>> Name2(pilota.rust_wrapper_arc="true"),
    3: required map<i32, list<A>> Name3(pilota.rust_wrapper_arc="true"),
}

service TestService {
    TEST(pilota.rust_wrapper_arc="true") test(1: TEST req(pilota.rust_wrapper_arc="true"));
}

service testService {
    TEST(pilota.rust_wrapper_arc="true") test(1: TEST req(pilota.rust_wrapper_arc="true"));
}
