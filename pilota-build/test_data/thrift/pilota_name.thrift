
struct TEST {
    1: required string ID,
}(pilota.name="Test2")

struct Test {
    1: required string ID,
    2: required string Id (pilota.name="hello"),
}