#[test]
fn ex01() {
    #[derive(Debug)]
    struct S(u8);
    impl S {
        fn f1(self) {
            println!("{self:?}");
        }
        fn f2(mut self) {
            self.0 += 1;
            println!("{self:?}");
        }
        fn f3(&self) {
            println!("{self:?}");
        }
        fn f4(&mut self) {
            self.0 *= 2;
            println!("{self:?}");
        }
    }

    fn method_calls() {
        let s1 = S(0);
        s1.f1();
        // println!("{s1:?}");
        let s2 = S(10);
        s2.f2();
        // println!("{s2:?}");
        let s3 = S(20);
        s3.f3();
        println!("{s3:?}");
        let mut s4 = S(30);
        s4.f4();
        println!("{s4:?}");
    }

    method_calls();
}

#[test]
fn ex02() {
    fn return_expressions() {
        fn returns_nothing() {
            return;
        }
        let r1 = returns_nothing();
        println!("r1 = {r1:?}");
        fn returns_something() -> u128 {
            return 42;
        }
        let r2 = returns_something();
        println!("r2 = {r2}");
    }

    return_expressions();
}
