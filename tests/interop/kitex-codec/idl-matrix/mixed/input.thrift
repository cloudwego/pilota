namespace go matrix
struct Message { 1: string value }
exception Failure { 1: string message }
typedef Message Alias
service Test {
        Message Plain(1: Message a)
        Message Unary(1: Message a) (streaming.mode="unary")
        Message Client(1: Message a) (streaming.mode="client")
        Message Server(1: Message a) (streaming.mode="server")
        Message Bidi(1: Message a) (streaming.mode="bidirectional")
    }
    service Child extends Test { Message Extra(1: Message a) }
    