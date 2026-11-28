#[test]
fn aliases_preserve_messages_and_field_impls() {
    use api_types::child_contract;

    let original = left_types::Value {
        number: 11,
        ..Default::default()
    };
    assert_eq!(original.as_ref(), &11);
    let request: child_contract::CallArg0 = original;
    let original: left_types::Value = request;
    assert_eq!(original.as_ref(), &11);

    let response: child_contract::CallReturn = right_types::Value {
        number: 13,
        ..Default::default()
    };
    let original: right_types::Value = response;
    assert_eq!(original.as_ref(), &13);

    let shared: child_contract::SharedArg0 = common_types::Value { number: 17 };
    let returned: child_contract::SharedReturn = shared;
    let original: common_types::Value = returned;
    assert_eq!(original.as_ref(), &17);
}
