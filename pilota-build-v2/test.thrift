namespace rust test

struct User {
    1: required i64 id,
    2: required string name,
    3: optional string email,
}

service UserService {
    User getUser(1: i64 id),
    void createUser(1: User user),
}