#[test]
fn format_macro_demo() {
    let s1 = format!("{} {} {}", 1, 2, 3);
    println!("{}", s1);
    let s2 = format!("{0} {0}", 10);
    println!("{0}", s2);
    let s3 = format!("{a}", a = 666);
    println!("{s3}");
}

#[test]
fn test_write_macro_demo() {
    use std::io::Error;
    use std::io::Write;
    pub fn write_macro_demo() -> Result<(), Error> {
        let mut buf = Vec::new();
        write!(&mut buf, "Hello {} times!", 42)?;
        // ...
        let text = String::from_utf8_lossy(&buf);
        println!("{text}");

        Ok(())
    }
    match write_macro_demo() {
        Ok(()) => println!("ok"),
        error => println!("{:?}", error),
    }
}

#[test]
fn panic_macro_demo_1() {
    panic!();
}

#[test]
fn panic_macro_demo_2() {
    panic!("Long time no see!");
}
#[test]
fn panic_macro_demo_3() {
    panic!("Well, it's been {} years.", 1000);
}

#[test]
fn assert_macros() {
    assert!(true == true, "True must be true");
    let t = 333;
    assert_eq!(333, t, "How can 333 be not {t}?");
    assert_ne!("hello", "world", "Is hello world?");
}

#[test]
fn dbg_macro_demo() {
    let a = 963;
    let x = dbg!(a + 36);
    assert_eq!(x, 999);
    let b = String::from("Rust");
    let y = dbg!(&b);
    println!("{y}");
}

pub fn todo_macro_demo(_x: i32, _y: f32) -> Option<bool> {
    todo!("to do or not to do?");
}

#[test]
fn vec_macro_demo() {
    let v1 = vec![1, 2, 3]; 
    let _v = vec![10; 3]; 
    assert_eq!([1, 2, 3].as_slice(), v1.as_slice());
    }