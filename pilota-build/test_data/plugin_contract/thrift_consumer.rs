#[test]
fn aliases_preserve_messages_inheritance_exceptions_and_field_impls() {
    use api_types::child_contract;

    let original = left_types::Value {
        number: 11,
        ..Default::default()
    };
    assert_eq!(original.as_ref(), &11);
    let request: child_contract::CallArg0 = original;
    let response: child_contract::CallReturn = request;
    let original: left_types::Value = response;
    assert_eq!(original.as_ref(), &11);

    let second: child_contract::CallArg1 = right_types::Value {
        number: 13,
        ..Default::default()
    };
    assert_eq!(second.as_ref(), &13);

    let shared: child_contract::SharedArg0 = common_types::Value { number: 17 };
    let returned: child_contract::SharedReturn = shared;
    let inherited: common_types::parent_contract::InheritedArg0 = returned;
    let inherited: child_contract::InheritedArg0 = inherited;
    let returned: child_contract::InheritedReturn = inherited;
    let original: common_types::Value = returned;
    assert_eq!(original.as_ref(), &17);

    let failure: child_contract::CallFailure = Failure { number: 7 };
    let exception: child_contract::CallException = api_types::ChildCallException::Failure(failure);
    let child_contract::CallException::Failure(failure) = exception;
    let original: Failure = failure;
    assert_eq!(original.as_ref(), &7);
}
