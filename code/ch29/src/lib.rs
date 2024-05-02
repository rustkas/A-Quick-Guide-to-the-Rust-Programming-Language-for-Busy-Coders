#[test]
fn ex01() {
    fn let_demo() {
        let x: u128 = 42;
        assert_eq!(x, 42);
        let y2 = (42, 24);
        let (y, _): (i8, u8) = y2;
        assert_eq!(y, 42);

        let w2 = [1, 1, 2, 3, 42, 8, 13];
        let [.., w, _, _]: [i32; 7] = w2;
        assert_eq!(w, 42);

        struct S {
            p1: i8,
            _p2: i8,
        }
        let z2 = S { p1: 42, _p2: 84 };
        let S { p1: z, _p2: _ }: S = z2;
        assert_eq!(z, 42);

        enum E {
            V(i32),
        }
        let e2 = E::V(42);
        let E::V(u) = e2;
        assert_eq!(u, 42);
    }

    let_demo();
}

#[test]
fn ex02() {
    fn expression_statements(a: i32) {
        42;
        if a > 42 {
            true
        } else {
            false
        };
        println!("Forty two!!!");
    }
    expression_statements(43);
}
