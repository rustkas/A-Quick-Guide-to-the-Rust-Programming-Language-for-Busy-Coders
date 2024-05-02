#[test]
fn ex01() {
    use std::thread::sleep;
    use std::time::Duration;

    fn loop_loop() {
        let mut counter = 1;
        loop {
            counter += 1;

            if counter == 3 {
                break;
            }
            sleep(Duration::from_millis(1000));
            println!("Slept another second");
        }
    }

    loop_loop();
}

#[test]
fn ex02() {
    fn for_loops() {
        let arr1 = ['C', 'a', 'T'];
        for a in arr1 {
            println!("{a}");
        }
        let arr2 = vec!["Hi", "World"];
        for a in &arr2 {
            println!("{a}");
        }
        let mut arr3 = [1, 2, 3, 5, 8];
        for a in &mut arr3 {
            *a += 1;
            println!("{a}");
        }
        println!("{arr3:?}");
    }

    for_loops();
}

#[test]
fn ex03() {
    fn for_range_loops() {
        for i in 5..=8 {
            print!("{i} , ");
        }
        println!();
        for c in 'c'..'t' {
            print!("{c}, ");
        }
        println!();
    }
    for_range_loops();
}

#[test]
fn ex04() {
    fn while_loops() {
        let (mut i, mut j) = (0, 0);
        while i + j < 5 {
            println!("Counting: {}", i + j);
            (i, j) = (i + 1, j + 1);
        }
        println!("Done: i = {i}, j = {j}");
    }
    while_loops();
}
#[test]
fn ex05() {
    fn while_let_loops() {
        println!();
        use std::time::{SystemTime, UNIX_EPOCH};
        fn random() -> Option<i32> {
            if let Ok(s) = SystemTime::now().duration_since(UNIX_EPOCH) {
                if s.as_micros() % 2 == 0 {
                    Some(1)
                } else {
                    None
                }
            } else {
                None
            }
        }
        while let Some(i) = random() {
            println!("{i}");
        }
    }

    while_let_loops();
}
#[allow(dead_code)]
#[test]
fn ex06() {
    #[derive(Debug)]
    enum Degree {
        Unknown,
        Celsius(f32),
        Fahrenheit(f32),
    }
    fn while_let_or_patterns() {
        use Degree::*;
        let mut degrees = vec![Celsius(100.), Unknown, Fahrenheit(37.)];
        println!("{degrees:?}");
        while let Some(Celsius(t)) | Some(Fahrenheit(t)) = degrees.pop() {
            println!("t = {t} C/F");
        }
    }
    while_let_or_patterns();
}

#[test]
fn ex07() {
    fn loop_labels() {
        #![allow(unused, while_true, irrefutable_let_patterns)]
        'loop1: loop {
            break 'loop1;
        }
        'loop2: while true {
            break 'loop2;
        }
        'loop3: while let age = 42 {
            println!("{age}");
            break 'loop3;
        }
        'loop4: for i in [1, 2, 3] {
            println!("{i}");
            if i == 2 {
                break 'loop4;
            }
        }
        'block5: {
            print!("Empty");
            break 'block5;
        }
    }
    loop_labels();
}

#[test]
fn ex08() {
    fn break_expressions_1() {
        let mut looped = 0;
        while looped < 42 {
            loop {
                looped += 10;
                if looped > 21 {
                    break;
                }
            }
            println!("{looped}");
        }
    }

    break_expressions_1();

    fn break_expressions_2() {
        let (mut x, mut y) = ('\0', 0u8);
        'fst: for i in 'a'..='z' {
            '_snd: for j in 1..=100 {
                if i != 'i' {
                    break;
                }
                if j == 21 {
                    x = ((i as u8) - 32) as char;
                    y = j + 21;
                    break 'fst;
                }
            }
        }
        println!("x = {x}, y = {y}");
    }

    break_expressions_2();

    fn break_expressions_3() {
        let z = 'x: loop {
            loop {
                break 'x 333;
            }
        };
        println!("z = {z}");
    }
    break_expressions_3();
}

#[test]
fn ex09() {
    fn continue_expressions() {
        let mut x = 0;
        'w1: while {x += 1; x} < 10 {
            let mut y = 0;
            while {y += 1; y} < 10 {
                let sum = x + y;
                if sum % 3 == 0 {
                    continue 'w1;
                } else if sum % 2 == 0 {
                    continue;
                }
                println!("sum = {sum}");
            }
        }
    }

    continue_expressions();
}
