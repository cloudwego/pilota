include!("../test_data/protobuf/default.rs");
#[test]
fn test_pb2_default() {
    let a = default::Defaults::default();
    assert_eq!(a.a, Some(42));
    assert_eq!(a.a64, Some(-9007199254740991));
    assert_eq!(a.b, Some(3.5));
    assert_eq!(a.c, Some(-1.25));
    assert_eq!(a.d, Some(true));
    assert_eq!(a.e, Some("hi".into()));
    assert_eq!(a.color, Some(default::Color::GREEN));
    assert_eq!(a.inner, default::outer::Inner::default());
    assert_eq!(a.zero, Some(0));
    assert_eq!(a.dzero, Some(0.0f64));
}
