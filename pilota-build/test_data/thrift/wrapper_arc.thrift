struct TEST {
    1: required string ID(pilota.rust_wrapper_arc="true"),
}

service TestService {
    string test(1: TEST req(pilota.rust_wrapper_arc="true"));
}