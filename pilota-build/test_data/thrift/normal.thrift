

struct A {
    1: i32 a,
}

struct b {
    2: A a,
}

struct SubMessage {
    2: optional string value;
}

struct Message {
	1: uuid uid;
    2: optional string value;
    3: optional list<SubMessage> subMessages;
}

struct ObjReq {
    1: required Message msg
    2: required map<Message, SubMessage> msgMap
    3: required list<SubMessage> subMsgs
    4: optional set<Message> msgSet
    5: required string flagMsg
    6: optional string mockCost,
}

exception STException {
    1: string message;
}

service Test {
   void test_123();
   ObjReq testException(1: ObjReq req) throws (1: STException stException);
}