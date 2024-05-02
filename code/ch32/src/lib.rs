#[test]
// The match Expressions
fn ex01() {
    fn match_expressions() {
        for rank in &['A', 'J', 'K', 'X'] {
            let value: Vec<u32> = match rank {
                'A' => vec![1, 11],
                'K' | 'Q' | 'J' | 'T' => vec![10],
                r @ ('2'..='9') => {
                    vec![r.to_digit(10).unwrap()]
                }
                _ => panic!("Invalid rank {rank}"),
            };
            println!("Rank: {rank}, Value: {value:?}");
        }
    }
    match_expressions();
}

#[test]
//
fn ex02() {
    fn match_guards() {
        fn radius(x: f32, y: f32) -> f32 {
            (x.powf(2.) + y.powf(2.)).sqrt()
        }
        let p = (1., 0.);
        match p {
            (x, y) if x == 0. && y == 0. => {
                println!("I'm the origin");
            }
            (x, y) if radius(x, y) == 1. => {
                println!("I'm on a unit circle");
            }
            (x, y) if x == -y => {
                println!(r#"I'm on an "anti-diagonal""#);
            }
            (x, y) => {
                println!("I'm somewhere, ({x}, {y})");
            }
        }
    }

    match_guards();
}

#[test]
fn ex03() {
    fn if_else_expressions() {
        let today = 30;
        let sign = if today >= 42 { 333 } else { 666 };
        if sign >= 999 {
            println!("Ominous");
        } else if sign >= 666 {
            println!("Not so lucky");
        } else {
            println!("Weird");
        }
    }
    if_else_expressions();
}

#[test]
fn ex04() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum Greek {
        Alpha(u8),
        Beta(u8),
        Omega,
    }
    fn if_let_expressions() {
        use Greek::*;
        let a = Alpha(42);
        if let Alpha(21) = a {
            println!("a is Alpha(21)");
        } else if let Beta(x) = a {
            println!("a is Beta({x})");
        } else if let Alpha(y) = a {
            println!("a is Alpha({y})");
        } else {
            println!("a is no Greek");
        }
    }
    if_let_expressions();

    fn ifs_and_if_lets() {
        use Greek::*;
        let value = Beta(84);
        let x = if let Alpha(_) = value {
            1000
        } else if Beta(42) == value {
            100
        } else if let Omega = value {
            10
        } else {
            0
        };
        println!("x = {x}");
    }
    ifs_and_if_lets();

    fn if_let_or_patterns() {
        use Greek::*;
        let a = Alpha(42);
        if let Alpha(21) | Beta(21) = a {
            println!("a contains 21");
        } else if let Alpha(x) | Beta(x) = a {
            println!("a contains {x}");
        }
    }
    if_let_or_patterns();
}

#[test]
fn ex05() {
    fn let_else_demo() {
        let food = Some("cookies");
        let Some(x) = food else {
            panic!("No cookies!!!");
        };
        println!("Finally, some {x}!");
    }
    let_else_demo();
}
