namespace go matrix
struct Message { 1: string value }
exception Failure { 1: string message }
typedef Message Alias
service Test { Message Call(1: map<string,Message> a) (streaming.mode="bidirectional") }
