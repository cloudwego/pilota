struct A {
    // A name that is a Rust keyword: the generated ident is escaped to `r#type`,
    // but the serialized name must stay `type`.
    1: required string type,
    // A name that is rewritten to a snake_case ident: `some_field`.
    2: required string SomeField,
    // An explicit rename wins over the generated one.
    3: required string c(pilota.serde_attribute = "#[serde(rename = \"CC\")]"),
    // An attribute that merely embeds the word "rename" does not count as one.
    4: required string d(pilota.serde_attribute = "#[serde(alias = \"renamed_value\")]"),
}
