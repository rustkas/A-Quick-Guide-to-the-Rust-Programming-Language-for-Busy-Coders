#[test]
fn ex01() {
    fn grouped_expressions() {
        let a = 1 + 2 * 3;
        let b = (1 + 2) * 3;
        assert_eq!(a, 7);
        assert_eq!(b, 9);
    }
    grouped_expressions();
}

#[test]
fn ex02() {
    struct Factory<'a> {
        ctor: fn(&'a str) -> String,
    }
    fn function_pointer() {
        let fac = Factory { ctor: String::from };
        let ferris = (fac.ctor)("Ferris");
        println!("Ferris = {ferris}");
    }

    function_pointer();
}
