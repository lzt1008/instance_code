use instance_code::InstanceCode;

#[test]
fn aaa() {
    #[derive(InstanceCode)]
    struct TestA {
        a: i32,
        b: i32,
    }

    let a = TestA { a: 1, b: 2 };
    let code = a.instance_code();

    println!("{}", code);
}
