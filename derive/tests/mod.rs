use std::sync::Arc;

use instance_code::InstanceCode;

#[test]
fn test_derive_enum() {
    #[derive(InstanceCode)]
    enum Test {
        A(u32),
        B(String),
    }

    let test_a = Test::A(42);
    let test_b = Test::B("Hello, World!".to_string());

    assert_eq!(test_a.instance_code().to_string(), "Test :: A (42u32 ,)");

    assert_eq!(
        test_b.instance_code().to_string(),
        "Test :: B (\"Hello, World!\" . to_owned () ,)"
    );
}

#[test]
fn test_derive_struct() {
    #[derive(InstanceCode)]
    struct Test {
        a: u32,
        b: String,
    }

    let test = Test {
        a: 42,
        b: "Hello, World!".to_string(),
    };

    assert_eq!(
        test.instance_code().to_string(),
        "Test { a : 42u32 , b : \"Hello, World!\" . to_owned () , }"
    );
}

#[test]
fn test_derive_recursive() {
    #[derive(InstanceCode)]
    enum Test {
        B { a: u32, b: TestInner },
    }

    #[derive(InstanceCode)]
    struct TestInner {
        a: u32,
        b: Arc<String>,
    }

    let test = Test::B {
        a: 42,
        b: TestInner {
            a: 42,
            b: Arc::new("Hello, World!".to_string()),
        },
    };

    assert_eq!(
        test.instance_code().to_string(),
        "Test :: B { a : 42u32 , b : TestInner { a : 42u32 , b : std :: sync :: Rc :: new (\"Hello, World!\" . to_owned ()) , } , }"
    );
}

#[test]
fn test_derive_with_path() {
    #[derive(InstanceCode)]
    #[instance(path = crate)]
    enum Test {
        A(u32),
    }

    let test = Test::A(42);

    let output = test.instance_code();

    assert_eq!(output.to_string(), "crate :: Test :: A (42u32 ,)");
}

#[test]
fn test_derive_with_multi_path() {
    #[derive(InstanceCode)]
    #[instance(path = my_crate::my_module)]
    enum Test {
        A(u32),
    }

    let test = Test::A(42);

    let output = test.instance_code();

    assert_eq!(
        output.to_string(),
        "my_crate :: my_module :: Test :: A (42u32 ,)"
    );
}
