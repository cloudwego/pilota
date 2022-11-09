
struct TEST {
    1: required string ID,
}(pilota.name="Test2")

const string id = "id" (pilota.name="LANG_ID");

struct Test {
    1: required string ID,
    2: required string Id (pilota.name="hello"),
}(pilota.name="Test1")

enum Index {
    A (pilota.name="AA"),
    B,
}

service TestService {
    Test test(1: TEST req);
}