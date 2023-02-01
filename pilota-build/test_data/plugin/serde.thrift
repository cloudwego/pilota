struct A {
    1: required string a(pilota.serde_attribute = "#[serde(rename = \"AA\")]"),
    2: required i32 b,
}(pilota.serde_attribute = "#[serde(rename_all = \"camelCase\")]")

typedef i32 B(pilota.serde_attribute = "#[serde(rename = \"BB\")]")

enum C {
    D(pilota.serde_attribute = "#[serde(rename = \"DD\")]"),
    E,
}(pilota.serde_attribute = "#[serde(untagged)]")
