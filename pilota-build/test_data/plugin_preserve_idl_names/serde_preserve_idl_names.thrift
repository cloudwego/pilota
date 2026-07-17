struct A {
    // Rust keyword: should become r#type, but renamed to "type"
    1: required string type,
    // A name that is rewritten to a snake_case ident: `some_field`.
    2: required string SomeField,
    // An explicit rename wins over the generated one.
    3: required string c(pilota.serde_attribute = "#[serde(rename = \"CC\")]"),
    // An attribute that merely embeds the word "rename" does not count as one.
    4: required string d(pilota.serde_attribute = "#[serde(alias = \"renamed_value\")]"),
    // A rename covering only serialization: deserialization is still spelled
    // with the Rust ident, so that direction alone gets the IDL name.
    5: required string SerOnly(pilota.serde_attribute = "#[serde(rename(serialize = \"EE\"))]"),
    // The mirror case: only serialization is left to preserve.
    6: required string DeOnly(pilota.serde_attribute = "#[serde(rename(deserialize = \"FF\"))]"),
    // Both directions renamed explicitly, so preserve has nothing to add.
    7: required string BothDirs(pilota.serde_attribute = "#[serde(rename(serialize = \"GS\", deserialize = \"GD\"))]"),
}

union TestUnion {
    1: string type,
    2: string some_field,
    3: string c(pilota.serde_attribute = "#[serde(rename = \"CC\")]"),
    4: string d(pilota.serde_attribute = "#[serde(alias = \"renamed_value\")]"),
    5: string ser_only(pilota.serde_attribute = "#[serde(rename(serialize = \"EE\"))]"),
    6: string de_only(pilota.serde_attribute = "#[serde(rename(deserialize = \"FF\"))]"),
    7: string both_dirs(pilota.serde_attribute = "#[serde(rename(serialize = \"GS\", deserialize = \"GD\"))]"),
}