
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

service TestService {
    Test test(1: TEST req);
    Test Test(1: TEST Req);
}
