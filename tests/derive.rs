use const_default::ConstDefault;

#[cfg(feature = "derive")]
#[derive(Debug, Default, ConstDefault, PartialEq)]
struct Test {
    f0: u32,
    f1: Option<bool>,
}

#[cfg(feature = "derive")]
#[test]
fn derive() {
    let t0: Test = ConstDefault::DEFAULT;
    let t1: Test = Default::default();

    assert_eq!(t0, t1);
}

#[cfg(feature = "derive")]
#[derive(Debug, Default, ConstDefault, PartialEq)]
struct TestGeneric<T> {
    f0: T,
    f1: Option<bool>,
}

#[cfg(feature = "derive")]
#[test]
fn derive_generic() {
    let t0: TestGeneric<i32> = ConstDefault::DEFAULT;
    let t1: TestGeneric<i32> = Default::default();

    assert_eq!(t0, t1);
}

#[test]
fn compare() {
    let t0 = i32::DEFAULT;
    let t1 = i32::default();

    assert_eq!(t0, t1);
}
