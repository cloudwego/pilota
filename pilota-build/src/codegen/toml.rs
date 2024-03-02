pub fn merge_tomls(a: &mut toml::Value, b: toml::Value) {
    match (a, b) {
        (toml::Value::Boolean(a), toml::Value::Boolean(b)) => {
            *a = b;
        }
        (toml::Value::String(a), toml::Value::String(b)) => *a = b,
        (toml::Value::Array(a), toml::Value::Array(b)) => {
            a.extend(b);
            a.sort_by_key(|a| a.to_string());
            a.dedup_by(|a, b| a.to_string() == b.to_string());
        }
        (toml::Value::Table(a), toml::Value::Table(b)) => b.into_iter().for_each(|(k, v)| {
            if a.contains_key(&k) {
                merge_tomls(a.get_mut(&k).unwrap(), v);
            } else {
                a.insert(k, v);
            }
        }),
        // maybe depend on specific version for testing, don't do anything
        (toml::Value::Table(_), toml::Value::String(_)) => {}
        pair => panic!("can not merge {pair:?}"),
    }
}
