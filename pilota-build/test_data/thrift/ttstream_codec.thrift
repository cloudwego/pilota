namespace rs codec

struct Nested {
    1: required i32 value
    2: optional string label
}

struct CodecMessage {
    1: i64 integer
    2: bool flag
    3: double real
    4: string text
    5: binary blob
    6: list<i32> numbers
    7: map<string, list<i64>> nested
    8: optional string note
    9: set<i16> ids
    10: optional Nested child
    11: optional list<Nested> children
    12: string default_text = "default"
}

typedef CodecMessage MessageAlias
