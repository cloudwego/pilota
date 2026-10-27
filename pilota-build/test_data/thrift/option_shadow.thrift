// Types whose Rust names shadow the prelude's `Some`, `None` and `Option`.
// The generated code must keep working with these in scope.

enum SOME {
    ONE = 1,
}

enum NONE {
    ZERO = 0,
}

enum OPTION {
    ANY = 0,
}

struct Inner {
    1: required i32 id,
}

struct Item {
    1: optional string name,
    2: required SOME some_code,
    3: required NONE none_code,
    4: optional OPTION option_code,
    5: optional list<i32> nums,
    6: optional map<string, string> tags,
    7: optional set<i32> ids,
    8: optional Inner inner,
    9: optional string with_default = "default",
    10: optional list<i32> lazy_default = [1, 2],
}

const Item DEFAULT_ITEM = {"name": "const"}

union Choice {
    1: string text,
    2: Item item,
}

exception ItemError {
    1: optional string message,
}

service ItemService {
    Item Get(1: Item req) throws (1: ItemError err),
    oneway void Fire(1: Choice choice),
}
