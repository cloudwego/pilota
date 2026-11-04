pub fn merge_tomls(a: &mut toml::Value, b: toml::Value) {
    match (a, b) {
        (toml::Value::Boolean(a), toml::Value::Boolean(b)) => {
            *a = b;
        }
        (toml::Value::String(_), toml::Value::String(b)) if b == "*" => {}
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
        // A transport can add required features to Cargo's shorthand dependency
        // form. Keep the user's version instead of replacing it with "*".
        (a @ toml::Value::String(_), toml::Value::Table(mut b))
            if b.get("version").and_then(toml::Value::as_str) == Some("*") =>
        {
            b.insert("version".into(), a.clone());
            *a = toml::Value::Table(b);
        }
        // maybe depend on specific version for testing, don't do anything
        (toml::Value::Table(_), toml::Value::String(_)) => {}
        pair => panic!("can not merge {pair:?}"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn required_transport_features_preserve_shorthand_dependency_versions() {
        let mut manifest: toml::Value =
            toml::from_str("[workspace.dependencies]\nvolo-grpc = \"0.12.2\"\n").unwrap();
        let required = toml::from_str(
            "[workspace.dependencies]\nvolo-grpc = { version = \"*\", features = [\"thrift\"] }\n",
        )
        .unwrap();
        super::merge_tomls(&mut manifest, required);
        let dependency = &manifest["workspace"]["dependencies"]["volo-grpc"];
        assert_eq!(dependency["version"].as_str(), Some("0.12.2"));
        assert_eq!(dependency["features"][0].as_str(), Some("thrift"));
    }

    #[test]
    fn required_transport_features_extend_configured_dependency_without_erasing_options() {
        let mut manifest: toml::Value = toml::from_str(
            "[workspace.dependencies]\nvolo-grpc = { version = \"0.12.2\", default-features = false, features = [\"rustls\"] }\n",
        ).unwrap();
        let required = toml::from_str(
            "[workspace.dependencies]\nvolo-grpc = { version = \"*\", features = [\"thrift\"] }\n",
        )
        .unwrap();
        super::merge_tomls(&mut manifest, required);
        let dependency = &manifest["workspace"]["dependencies"]["volo-grpc"];
        assert_eq!(dependency["version"].as_str(), Some("0.12.2"));
        assert_eq!(dependency["default-features"].as_bool(), Some(false));
        assert_eq!(
            dependency["features"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<Vec<_>>(),
            ["rustls", "thrift"]
        );
    }
}
