struct Item {
    1: required i64 id,
    2: required string title,
}

const Item default_item = {'id': 1, 'title': "a"}
const map<string, Item> default_item_map = {'a': {'id': 1, 'title': "a"}}

struct GetItemRequest {
    1: required i64 id,
    2: optional Item item_opt = default_item,
    3: optional Item item_opt2 = {'id': 1, 'title': "a", 'content': "b"},
    4: required map<string, string> test_map,
    5: required map<i64, string> test_map2,
}
