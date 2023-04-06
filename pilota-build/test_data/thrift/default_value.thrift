struct A {
    1: required string faststr = "hello world",
    2: required string string = "test"(pilota.rust_type = "string"),
}