#[test]
fn ex01() {
    fn negation_operator() {
        let (x, y, z) = (0, 10, 22.2);
        assert_eq!((-x, -y, -z), (0, -10, -22.2));
    }

    negation_operator();
}

#[test]
fn ex02() {
    fn logical_not_operator() {
        let (b1, b2) = (true, false);
        assert_eq!((!b1, !!b1, !b2, !!b2), (false, true, true, false));
    }
    logical_not_operator();
}

#[test]
fn ex03() {
    fn bitwise_not_operator() {
        let u1 = 0b01001u8;
        println!("u1 -> !u1 = {0:b} ({0}) -> {1:b} ({1})", u1, !u1);
        let i1 = 0b01001i8;
        println!("i1 -> !i1 = {0:b} ({0}) -> {1:b} ({1})", i1, !i1);
    }

    bitwise_not_operator();
}
#[test]
fn ex04() {
    fn arithmetic_demo() {
        println!("5 + 2 = {a}", a = 5 + 2);
        println!("5 * 2 = {m}", m = 5 * 2);
        println!("5 / 2 = {d}", d = 5 / 2);
        println!("-5 / 2 = {d}", d = -5 / 2);
        println!("5 % 2 = {r}", r = 5 % 2);
        println!("5 % -2 = {r}", r = 5 % -2);
        println!("-5 % 2 = {r}", r = -5 % 2);
        println!("-5 % -2 = {r}", r = -5 % -2);
    }

    arithmetic_demo();
}

#[test]
fn ex05() {
    fn logical_demo() {
        for l in [true, false] {
            for r in [true, false] {
                print!("{l} & {r} = {b}\t", b = l & r);
            }
            println!();
        }
        for l in [true, false] {
            for r in [true, false] {
                print!("{l} | {r} = {b}\t", b = l | r);
            }
            println!();
        }
        for l in [true, false] {
            for r in [true, false] {
                print!("{l} ^ {r} = {b}\t", b = l ^ r);
            }
            println!();
        }
    }
    logical_demo();
}

#[test]
fn ex06() {
    fn lazy_boolean_operators() {
        for l in [true, false] {
            for r in [true, false] {
                let b = {
                    print!("(l)");
                    l
                } && {
                    print!("(r)");
                    r
                };
                print!("\t");
                print!("{l} && {r} = {b}\t");
            }
            println!();
        }
        for l in [true, false] {
            for r in [true, false] {
                let b = {
                    print!("(l)");
                    l
                } || {
                    print!("(r)");
                    r
                };
                print!("\t");
                print!("{l} || {r} = {b}\t");
            }
            println!();
        }
    }

    lazy_boolean_operators();
}

#[test]
fn ex07() {
    fn bitwise_shift_operators() {
        let u1 = 0b1000101u8;
        println!("u1 -> (u1 << 1): {0:b} ({0}) -> {1:b} ({1})", u1, (u1 << 1));
        println!("u1 -> (u1 >> 2): {0:b} ({0}) -> {1:b} ({1})", u1, (u1 >> 2));
        let i1 = 0b1000101i8;
        println!("i1 -> (i1 << 1): {0:b} ({0}) -> {1:b} ({1})", i1, (i1 << 1));
        println!("i1 -> (i1 >> 2): {0:b} ({0}) -> {1:b} ({1})", i1, (i1 >> 2));
        let i1 = -0b1000101i8;
        println!("i1 -> (i1 << 1): {0:b} ({0}) -> {1:b} ({1})", i1, (i1 << 1));
        println!("i1 -> (i1 >> 2): {0:b} ({0}) -> {1:b} ({1})", i1, (i1 >> 2));
    }
    bitwise_shift_operators();
}

#[test]
fn ex08() {
    fn assignment_operator() {
        let mut a = 10;
        #[allow(dropping_copy_types)]
        drop(a);
        let b = 1;
        a = b + 3;
        println!("{a}");
        let mut _x = 5;
        let _y;
        #[allow(unused_parens)]
        let z = _y = (_x = 30);
        println!("{z:?}");
    }
    assignment_operator();
}

#[test]
fn ex09() {
    fn comparison_demo() {
        let x = String::from("Hello ");
        let y = String::from("World!");
        if x == y {
            panic!("Hello Huh?");
        }
        if PartialEq::eq(&x, &y) {
            panic!("World Wot?");
        }
        println!("{x}{y}");
    }
    comparison_demo();
}

#[test]
fn ex10() {
    fn shared_borrow_demo() {
        let x = 101;
        let y = &x;
        let z = &&x;
        assert_eq!(y, &x);
        assert_eq!(z, &y);
    }
    shared_borrow_demo();
}
#[test]
fn ex11() {
    fn mutable_borrow_demo() {
        let (mut x1, mut x2, x3, mut x4) = (201, 301, 401, 501);
        {
            let y1 = &mut x1;
            let z2 = &&mut x2;
            let z3 = &mut &x3;
            let z4 = &mut &mut x4;
            // println!("{x1},{x2},{x3},{x4}");
            println!("{y1}, {z2}, {z3}, {z4}");
        }
        println!("{x1}, {x2}, {x3}, {x4}");
    }
    mutable_borrow_demo();
}
