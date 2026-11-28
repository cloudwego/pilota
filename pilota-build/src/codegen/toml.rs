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
            if k == "version"
                && v.as_str() == Some("*")
                && ["path", "git", "workspace"]
                    .iter()
                    .any(|source| a.contains_key(*source))
            {
                return;
            }
            if a.contains_key(&k) {
                merge_tomls(a.get_mut(&k).unwrap(), v);
            } else {
                a.insert(k, v);
            }
        }),
        // A transport can add required features to Cargo's shorthand dependency
        // form. Keep the user's version instead of replacing it with "*".
        (a @ toml::Value::String(_), b @ toml::Value::Table(_)) => {
            *a = toml::Value::Table(toml::map::Map::from_iter([("version".into(), a.clone())]));
            merge_tomls(a, b);
        }
        // maybe depend on specific version for testing, don't do anything
        (toml::Value::Table(_), toml::Value::String(_)) => {}
        pair => panic!("can not merge {pair:?}"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dependency_sources_and_shorthand_versions_survive_feature_merging() {
        for source in [
            r#"path = "../runtime""#,
            "git = \"https://example.com/runtime\"\nrev = \"abc\"",
            "workspace = true",
        ] {
            let mut dependency: toml::Value = toml::from_str(source).unwrap();
            super::merge_tomls(
                &mut dependency,
                toml::from_str("version = \"*\"\nfeatures = [\"thrift\"]").unwrap(),
            );
            assert!(dependency.get("version").is_none(), "{dependency}");
            assert_eq!(dependency["features"][0].as_str(), Some("thrift"));
        }
        for (extra, expected) in [
            ("", "0.12.2"),
            ("version = \"*\"", "0.12.2"),
            ("version = \"0.13\"", "0.13"),
        ] {
            let mut dependency = toml::Value::String("0.12.2".into());
            super::merge_tomls(
                &mut dependency,
                toml::from_str(&format!("{extra}\nfeatures = [\"thrift\"]")).unwrap(),
            );
            assert_eq!(dependency["version"].as_str(), Some(expected));
            assert_eq!(dependency["features"][0].as_str(), Some("thrift"));
        }
    }
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
