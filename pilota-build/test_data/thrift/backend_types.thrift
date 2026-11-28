namespace rs rust_types
union Event { 1:i64 value, 2:string label }
typedef Event Alias
struct Node { 1:optional Node next }
struct EmptyPayload {}
struct RecursivePayload { 1:optional RecursivePayload next }
service Types {
    oneway void Notify(1:Node node)
    void Zero()
    i32 Scalar(1:i32 value)
    RecursivePayload ProtocolName(1:RecursivePayload value)
    EmptyPayload Shadow(1:EmptyPayload value)
    list<i64> Values(1:list<i64> values)
    Alias Watch(1:Alias event) (streaming.mode="server",pilota.name="watch_events")
} (pilota.rust_wrapper_arc="true")
