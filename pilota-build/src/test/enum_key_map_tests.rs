use pilota::{AHashMap, FastStr};

// 包含生成的测试代码
include!("../../test_data/thrift_with_field_mask/enum_key_map.rs");

// 使用正确的模块路径
use std::collections::{HashMap, HashSet};

use enum_key_map::enum_key_map::{EnumKeyMapTest, Item, Status};

fn create_test_data() -> EnumKeyMapTest {
    let mut status_map: AHashMap<Status, FastStr> = AHashMap::new();
    status_map.insert(Status::ACTIVE, FastStr::from_static_str("active_status"));
    status_map.insert(
        Status::INACTIVE,
        FastStr::from_static_str("inactive_status"),
    );

    let mut status_item_map: AHashMap<Status, Item> = AHashMap::new();
    status_item_map.insert(
        Status::ACTIVE,
        Item {
            id: 100,
            name: FastStr::from_static_str("active_item"),
            _field_mask: None,
        },
    );
    status_item_map.insert(
        Status::PENDING,
        Item {
            id: 200,
            name: FastStr::from_static_str("pending_item"),
            _field_mask: None,
        },
    );

    let mut status_list_map: AHashMap<Status, Vec<Item>> = AHashMap::new();
    status_list_map.insert(
        Status::ACTIVE,
        vec![
            Item {
                id: 1,
                name: FastStr::from_static_str("list_item_1"),
                _field_mask: None,
            },
            Item {
                id: 2,
                name: FastStr::from_static_str("list_item_2"),
                _field_mask: None,
            },
        ],
    );

    EnumKeyMapTest {
        status_map,
        status_item_map,
        status_list_map: Some(status_list_map),
        _field_mask: None,
    }
}

#[test]
fn test_enum_key_roundtrip_conversion() {
    let status = Status::ACTIVE;

    // 测试 enum 到 i32 的转换
    let i32_val: i32 = status.into();
    assert_eq!(i32_val, 1);

    // 测试 i32 到 enum 的转换
    let status_from_i32 = Status::from(2);
    assert_eq!(status_from_i32, Status::INACTIVE);

    // 测试 inner 方法
    assert_eq!(status.inner(), 1);

    // 测试 try_from_i32
    assert_eq!(Status::try_from_i32(3), Some(Status::PENDING));
    assert_eq!(Status::try_from_i32(999), None);
}

#[test]
fn test_enum_key_map_data_integrity() {
    let test_data = create_test_data();

    // 验证创建的数据完整性
    assert_eq!(test_data.status_map.len(), 2);
    assert_eq!(test_data.status_item_map.len(), 2);
    assert!(test_data.status_list_map.is_some());

    // 验证具体值
    assert_eq!(
        test_data.status_map.get(&Status::ACTIVE).unwrap(),
        &FastStr::from_static_str("active_status")
    );

    let active_item = test_data.status_item_map.get(&Status::ACTIVE).unwrap();
    assert_eq!(active_item.id, 100);
    assert_eq!(active_item.name, FastStr::from_static_str("active_item"));

    let pending_item = test_data.status_item_map.get(&Status::PENDING).unwrap();
    assert_eq!(pending_item.id, 200);
    assert_eq!(pending_item.name, FastStr::from_static_str("pending_item"));

    // 验证 list map
    let list_items = test_data
        .status_list_map
        .as_ref()
        .unwrap()
        .get(&Status::ACTIVE)
        .unwrap();
    assert_eq!(list_items.len(), 2);
    assert_eq!(list_items[0].id, 1);
    assert_eq!(list_items[1].id, 2);
}

#[test]
fn test_enum_key_hashable_and_comparable() {
    // 测试 enum 可以作为 HashMap 的 key
    let mut map: HashMap<Status, String> = HashMap::new();
    map.insert(Status::ACTIVE, "active".to_string());
    map.insert(Status::INACTIVE, "inactive".to_string());

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&Status::ACTIVE), Some(&"active".to_string()));

    // 测试 enum 可以在 HashSet 中
    let mut set: HashSet<Status> = HashSet::new();
    set.insert(Status::ACTIVE);
    set.insert(Status::PENDING);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&Status::ACTIVE));

    // 测试 enum 比较
    assert_eq!(Status::ACTIVE, Status::ACTIVE);
    assert_ne!(Status::ACTIVE, Status::INACTIVE);

    // 测试排序（enum 应该支持 Ord）
    let mut statuses = vec![Status::PENDING, Status::ACTIVE, Status::INACTIVE];
    statuses.sort();
    assert_eq!(statuses[0], Status::ACTIVE); // value=1
    assert_eq!(statuses[1], Status::INACTIVE); // value=2
    assert_eq!(statuses[2], Status::PENDING); // value=3
}

#[test]
fn test_enum_key_map_clone_and_debug() {
    let test_data = create_test_data();

    // 测试 clone
    let cloned = test_data.clone();
    assert_eq!(cloned.status_map.len(), test_data.status_map.len());
    assert_eq!(
        cloned.status_item_map.len(),
        test_data.status_item_map.len()
    );

    // 验证 clone 后的数据独立
    assert_eq!(
        cloned.status_map.get(&Status::ACTIVE),
        test_data.status_map.get(&Status::ACTIVE)
    );

    // 测试 debug 输出（确保不会 panic）
    let debug_str = format!("{:?}", cloned);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("EnumKeyMapTest"));
}

#[test]
fn test_enum_key_default_and_partial_construction() {
    // 测试默认值
    let default_item = Item::default();
    assert_eq!(default_item.id, 0);
    assert_eq!(default_item.name, "");
    assert!(default_item._field_mask.is_none());

    // 测试部分构造
    let partial_item = Item {
        id: 42,
        ..Default::default()
    };
    assert_eq!(partial_item.id, 42);
    assert_eq!(partial_item.name, "");

    // 测试部分构造的 struct
    let partial_test = EnumKeyMapTest {
        status_map: {
            let mut m = AHashMap::new();
            m.insert(Status::PENDING, FastStr::from_static_str("pending"));
            m
        },
        ..Default::default()
    };
    assert_eq!(partial_test.status_map.len(), 1);
    assert!(partial_test.status_item_map.is_empty());
    assert!(partial_test.status_list_map.is_none());
}

#[test]
fn test_enum_key_iteration_patterns() {
    let test_data = create_test_data();

    // 测试迭代所有 key-value 对
    let mut keys: Vec<&Status> = Vec::new();
    for key in test_data.status_map.keys() {
        keys.push(key);
    }
    assert_eq!(keys.len(), 2);

    // 测试过滤迭代
    let active_values: Vec<&FastStr> = test_data
        .status_map
        .iter()
        .filter(|(k, _)| **k == Status::ACTIVE)
        .map(|(_, v)| v)
        .collect();
    assert_eq!(active_values.len(), 1);
    assert_eq!(*active_values[0], "active_status");

    // 测试修改迭代
    let mut modified_data = create_test_data();
    for (key, item) in &mut modified_data.status_item_map {
        if *key == Status::ACTIVE {
            item.id += 1000; // ACTIVE 的 id 增加 1000
        }
    }

    let active_modified = modified_data.status_item_map.get(&Status::ACTIVE).unwrap();
    assert_eq!(active_modified.id, 1100); // 原来是 100

    let pending_unmodified = modified_data.status_item_map.get(&Status::PENDING).unwrap();
    assert_eq!(pending_unmodified.id, 200); // 保持不变
}

#[test]
fn test_enum_key_map_equality() {
    let data1 = create_test_data();
    let data2 = create_test_data();

    // 相同数据应该相等
    assert_eq!(data1, data2);

    // 不同数据不应该相等
    let mut different_data = create_test_data();
    different_data.status_map.clear();
    assert_ne!(data1, different_data);

    // 修改某个字段后不应该相等
    let mut modified_data = create_test_data();
    *modified_data.status_map.get_mut(&Status::ACTIVE).unwrap() =
        FastStr::from_static_str("modified");
    assert_ne!(data1, modified_data);
}
