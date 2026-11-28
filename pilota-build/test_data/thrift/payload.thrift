include "payload_types.thrift"

namespace rs payload_service

struct Empty {}

service Interop extends payload_types.Parent {
    payload_types.Message Unary(1: payload_types.Message request),
    payload_types.Message ExplicitUnary(1: payload_types.Message request) (streaming.mode="unary"),
    void Zero(),
    i64 Multi(2: i32 left, 9: i64 right),
    list<i64> Container(7: list<i64> values),
    payload_types.Message Throw(1: payload_types.Message request)
        throws (2: payload_types.Failure failure, 7: payload_types.Other other),
    void VoidThrow() throws (4: payload_types.Failure failure),
    payload_types.Message Client(1: payload_types.Message request) (streaming.mode="client"),
    payload_types.Message Server(1: payload_types.Message request) (streaming.mode="server"),
    payload_types.Message Bidi(1: payload_types.Message request) (streaming.mode="bidirectional"),
}
