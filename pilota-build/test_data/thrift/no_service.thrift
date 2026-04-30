struct Contact {
    1: required string email,
    2: required bool verified,
}

struct Person {
    1: required string name,
    2: required i32 age,
    3: required Contact contact,
}
