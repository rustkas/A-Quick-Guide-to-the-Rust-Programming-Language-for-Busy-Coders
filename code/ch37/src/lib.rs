#[test]
fn ex01() {
    fn process_exit_demo() {
        println!("Hello and Bye! :)");
        std::process::exit(0);
    }
    process_exit_demo();
}

#[test]
fn assert_demo() {
    let a = 1 + 2;
    assert_eq!(a, 3);
    let x = 10;
    let b = if 100 > x { true } else { false };
    assert!(b);
}

#[test]
fn ex02() {
    #[derive(Debug)]
    struct Error1(u8);
    #[derive(Debug)]
    struct Error2(u16);

    impl From<Error1> for Error2 {
        fn from(value: Error1) -> Self {
            Error2(value.0 as u16)
        }
    }

    fn error_1() -> Result<i32, Error1> {
        Err(Error1(1))
    }

    fn error_2() -> Result<i32, Error2> {
        let x = error_1()?;
        println!("{x}");
        Ok(0)
    }
    let error1 = Error1(8 as u8);
    let error2: Error2 = error1.into();
    println!("error2 = {}",error2.0);
    _ = error_1();
    _ = error_2();
}
