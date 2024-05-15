enum Index {
    A = 0x01,
    B = 0x10,
}

enum Err {

}

enum Ok {
    
}

service Test {
   Err test_enum(1: Ok req);
}