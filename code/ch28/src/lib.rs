#[test]
fn pattern_matching() {
    fn match_demo() {
        let legs = 2;
        match legs {
            2 => println!("A biped"),
            3 | 4 => println!("Most likely, a quadruped"),
            _ => println!("Not sure what it is"),
        }
    }
    match_demo();

    if let (_x, 5) = (1, 2) {}
}
#[test]
// Irrefutability
fn irrefutability() {
    #[allow(irrefutable_let_patterns)]
    if let (_x, _y) = (3, 4) {}
}

#[test]
//Destructuring
fn destructuring() {
    #[allow(dead_code)]
    struct Fruits {
        apple: i32,
        orange: i32,
        pear: i32,
    }
    fn destructure() {
        let fruits = Fruits {
            apple: 1,
            orange: 2,
            pear: 3,
        };
        let Fruits {
            apple: apple_count,
            orange: _,
            pear,
        } = fruits;
        println!("Apples: {apple_count}, Pears: {pear}");
    }
    destructure();
}

#[test]
// Literal Patterns
fn literal_patterns() {
    fn literal_patterns() {
        for i in -1..=2 {
            match i {
                1 => println!("I am positive!"),
                -1 => println!("I am negative!"),
                _ => println!("I am nobody!"),
            }
        }
    }
    literal_patterns();
}

#[test]
// Range Patterns
fn range_patterns() {
    fn range_patterns() {
        let c = 'm';
        let 'i'..='n' = c else {
            panic!("No match!");
        };
        let i = 101;
        if let 100.. = i {
            println!("Bigger than 100: {i}");
        }
    }
    range_patterns();
}

#[test]
// Wildcard Patterns
fn wildcard_patterns() {
    fn wildcard_patterns() {
        for pos in [Some((0, 0)), Some((3, 5)), None] {
            if let Some((x, _)) = pos {
                print!("X is {x},\t");
            } else {
                print!("None,\t");
            }
            if let Some(p) = pos {
                let y = (|_: i32, y: i32| y)(p.0, p.1);
                println!("Y is {y}");
            } else {
                println!("None");
            }
        }
    }
    wildcard_patterns();
}
#[allow(dead_code)]
const X: i32 = 42;

#[allow(dead_code)]
const Y: i32 = 21;
#[test]
// Path Patterns
fn path_patterns() {
    fn path_patterns() {
        let x = 42;
        match x {
            X => println!("Matched X"),
            self::Y => println!("Matched Y"),
            _ => println!("No match!"),
        }
        let z2 = Some::<i32>(42);
        if let None::<i32> = z2 {
            println!("z2 is None::<i32>");
        }
        for z in [Some::<i32>(42), None::<i32>] {
            match z {
                Option::None::<i32> => {
                    println!("Matched None::<i32>");
                }
                v => {
                    println!("Match any value {v:?}");
                }
            }
        }
    }

    path_patterns();
}

#[test]
// Identifier Patterns
fn identifier_patterns() {
    fn identifier_patterns() {
        let mut var1: f64 = 1.;
        let var2: f64 = 100.55;
        fn multiply(a: &mut f64, b: f64) -> f64 {
            *a = *a * b;
            *a
        }
        let prod = multiply(&mut var1, var2);
        println!("Prod = {prod}, Var1 = {var1}");
    }
    identifier_patterns();
}

#[test]
// ref Identifier Patterns
fn ref_identifier_pattern() {
    fn let_ref_bindings() {
        let ref var1: f32 = 1.0f32;
        let ref mut var2: f32 = 2.0f32;
        *var2 = 3.;
        println!("var1 = {var1}; var2 = {var2}");
    }

    let_ref_bindings();
}

#[test]
// ref Identifier Patterns
fn ref_identifier_pattern2() {
    struct Car {
        make: String,
        model: String,
    }
    fn ref_identifier_patterns() {
        let my_car = Car {
            make: String::from("De"),
            model: String::from("Lorean"),
        };
        match my_car {
            Car {
                make: ref _make,
                model: _model,
            } => {
                println!("{}", my_car.make);
                // println!("{}", my_car.model); ④
            }
        }
    }

    ref_identifier_patterns();
}

#[test]
// ref Identifier Patterns
fn ref_identifier_pattern3() {
    struct Car {
        make: String,
        model: String,
    }

    fn ref_binding_mode() {
        let your_car = Car {
            make: String::from("Trans"),
            model: String::from("Former"),
        };
        let car_or_not: Option<Car> = Some(your_car);
        if let Some(c) = &car_or_not {
            println!("{}{}", c.make, c.model);
            let yc = car_or_not.unwrap();
            println!("{}{}", yc.make, yc.model);
        }
    }
    ref_binding_mode();
}

#[test]
// Reference Patterns
fn reference_patterns() {
    fn reference_patterns() {
        let a = &"Hello, Underworld!";
        let &"Hello, Underworld!" = a else {
            panic!("The end of the world!");
        };
        let mut b = &Some("Hello, Overlord!");
        if let &mut &Some(x) = &mut b {
            println!("x = {x}");
            return;
        };
        panic!("The end of the kingdom!");
    }

    reference_patterns();
}

#[test]
// OR Patterns
fn or_patterns() {
    fn or_patterns() {
        let x: u8 = 3;
        match x {
            1 | 3 | 5 => {
                println!("Small and odd")
            }
            0 | 2..=9 => {
                println!("Small and even")
            }
            _ => println!("Big and hairy"),
        }
    }

    or_patterns();
}

#[test]
// Grouped Patterns
fn group_paterns() {
    fn grouped_patterns() {
        let a = &"ABC";
        if let &("ABC" | "XYZ") = a {
            println!("ABC or XYZ");
        }
    }

    grouped_patterns();
}

#[test]
// The Rest Patterns
fn rest_patterns() {
    fn rest_patterns() {
        let t = (1, 2, 3, 4);
        match t {
            (1, .., 3) => println!("First and last elements are 1 and 3"),

            (.., 4) => println!("The last element is 4"),
            (1, ..) => println!("The first element is 1"),
            (..) => println!("Anything goes"),
        }
    }
    rest_patterns();
}

#[test]
// Tuple Patterns
fn tuple_patterns() {
    let t = (1, true, 'a');
    match t {
        (_, false, _) => {
            println!("The second element is false");
        }
        (1, _, 'b') => {
            println!("First element is 1. Third element is 'b'");
        }
        (2, ..) => {
            println!("The first element is 2");
        }
        (a, b, c) => {
            println!("({a}, {b}, {c})");
        }
    }
}

#[test]
// Struct Patterns
fn struct_patterns() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    fn struct_patterns() {
        let p = Point {
            x: 10,
            y: 10,
            z: 20,
        };
        match p {
            Point { y: 0, .. } => {
                println!("I'm on the x-axis")
            }
            Point { x: 0, .. } => {
                println!("I'm on the y-axis")
            }
            Point { x, y, z: _ } if x == y => {
                println!("I'm diagonal: {x}")
            }
            Point { x, y, z } => {
                println!("({x}, {y}, {z})")
            }
        }
    }
    struct_patterns();
}

#[test]
// Tuple Struct Patterns

fn tuple_struct_pattern() {
    struct Coord(f32, f32);
    fn tuple_struct_patterns() {
        let c = Coord(0.1, 2.0);
        match c {
            Coord(lat, lon) if lon.abs() < 0.1 => {
                println!("Latitude is {lat}. Longitude is close to 0.");
            }
            Coord(lat, lon) if lat.abs() < 0.1 => {
                println!("Latitude is close to 0. Longitude is {lon}.");
            }
            _ => println!("An arbitrary coordinate."),
        }
        let Coord(lat, lon) = c;
        println!("Latitude is {lat}. Longitude is {lon}.");
    }
    tuple_struct_patterns();
}

#[test]
// Slice Patterns
fn slice_patterns() {
    fn slice_patterns() {
        let s = [1, 2, 3];
        let l = length(&s);
        println!("Length = {l}");
    }
    fn length<T: std::fmt::Debug>(s: &[T]) -> usize {
        match s {
            [] => 0,
            [a] => {
                println!("One element: {a:?}");
                1
            }
            [_, tail @ ..] => 1 + length::<_>(tail),
        }
    }

    slice_patterns();
}

#[test]
// At (@) Patterns
fn at_subpattern() {
    fn at_patterns() {
        for x in [2, 10] {
            if let var1 @ 1..=5 = x {
                println!("Var1 = {var1}");
            } else {
                println!("No match!");
            }
        }
    }
    at_patterns();
}
