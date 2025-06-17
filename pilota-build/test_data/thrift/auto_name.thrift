
struct TEST {
    1: required string Id,
}

const string ip = "ip";
const string IP = "IP";

struct Test {
    1: required string ID,
    2: required string Id,
}

enum Index {
    A,
    a,
}

struct TestException {

}

service Service {
    Test(pilota.rust_wrapper_arc="true") test(1: TEST req(pilota.rust_wrapper_arc="true")  , 2: TEST Req) throws (1: TestException e);
    Test Test(1: TEST Req) throws (1: TestException e);
    Test Test2(1: TEST type);
} (pilota.rust_wrapper_arc="true")

service service {
    Test test(1: TEST req) throws (1: TestException e);
    Test Test(1: TEST Req) throws (1: TestException e);
    Test Test2(1: TEST self);
}
