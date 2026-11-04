namespace go matrix
struct Message { 1: string value }
exception Failure { 1: string message }
typedef Message Alias
service Test { oneway void Call(1: Message a) (streaming.mode="bidirectional") }
