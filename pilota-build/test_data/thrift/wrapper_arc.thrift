struct TEST {
    1: required string ID(pilota.rust_wrapper_arc="true"),
    2: required list<list<i32>> Name2(pilota.rust_wrapper_arc="true"),
    3: required map<i32, list<i32>> Name3(pilota.rust_wrapper_arc="true"),
}

service TestService {
    string test(1: TEST req(pilota.rust_wrapper_arc="true"));
}