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
    // Already PascalCase: should stay StringValue
    1: string StringValue,

    // Rust keyword: should become Pub, but serialized name must stay "pub"
    2: i32 pub,

    // Rewritten to IntValue but renamed to int_value
    3: string int_value,
}