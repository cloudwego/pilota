namespace go matrix
struct Message { 1: string value }
exception Failure { 1: string message }
typedef Message Alias
service Test { Alias Call(1: Alias a) (streaming.mode="bidirectional") }
