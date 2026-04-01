enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

struct NestedItem {
    1: required i32 value,
    2: optional string label,
}

struct ComplexEnumKeyMapTest {
    1: required map<Priority, i32> priority_counts,
    2: required map<Priority, NestedItem> priority_items,
    3: required map<Priority, map<string, i32>> nested_maps,
    4: optional map<Priority, list<NestedItem>> priority_item_lists,
}
