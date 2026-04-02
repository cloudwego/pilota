enum Status {
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

struct Item {
    1: required i64 id,
    2: required string name,
}

struct EnumKeyMapTest {
    1: required map<Status, string> status_map,
    2: required map<Status, Item> status_item_map,
    3: optional map<Status, list<Item>> status_list_map,
}
