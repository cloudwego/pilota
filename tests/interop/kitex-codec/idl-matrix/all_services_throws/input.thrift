namespace go matrix
struct Message { 1: string value }
exception Failure { 1: string message }
typedef Message Alias
service Good { Message Plain(1: Message a) }
    service ThrowsService {
        Message Fail(1: Message a) throws (1: Failure f) (streaming.mode="client")
    }
    