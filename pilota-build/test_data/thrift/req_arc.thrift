struct TEST {
    1: required string ID,
}

service TestService {
    string test(1: TEST req(pilota.rust_type = "Arc"));
}