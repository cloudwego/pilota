
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
    Test test(1: TEST req, 2: TEST Req) throws (1: TestException e);
    Test Test(1: TEST Req) throws (1: TestException e);
    Test Test2(1: TEST type);
}

service service {
    Test test(1: TEST req) throws (1: TestException e);
    Test Test(1: TEST Req) throws (1: TestException e);
    Test Test2(1: TEST self);
}
