pub fn array_demo() {
    let arr1 = [1, 2, 3, 5];
    println!("arr1 = {arr1:?}");
    println!("arr1[1] = {}", arr1[1]);
    // println!("arr1[5] = {}", arr1[5]);
    let mut arr2 = [2u8, 4, 8, 10];
    println!("arr2 = {arr2:?}");
    arr2[2] = 88;
    println!("arr2[2] = {}", arr2[2]);
}

pub fn array_get() {
    let arr = ["Hell", "World", "Rust"];
    for i in [0, 2, 4] {
        if let Some(x) = arr.get(i) {
            println!("arr[{i}] == {x}");
        } else {
            println!("No value at index {i}");
        }
    }
}

pub fn array_expressions() {
    let a1 = ["Feliz", "Ferris"];
    let a2 = ["Crab"; 4];
    println!("{a1:?}");
    println!("{a2:?}");
}

pub fn slice_from_array() {
    let arr = [5, 10, 20, 42];
    let s1 = &arr as &[i32];
    println!("s1 = {s1:?}");
    let s2 = &arr[1..=2];
    println!("s2 = {s2:?}");
}

pub fn slice_from_array_2() {
    let mut arr: [u8; 4] = [5, 10, 20, 42];
    let m1 = &mut arr as &mut [u8];
    println!("m1 = {m1:?}");
    m1[1] = 11;
    println!("m1 = {m1:?}");
    println!("arr = {arr:?}");
}

pub fn slice_iteration() {
    let list = vec!["Do", "Re", "Mi", "Fa"];
    for v in &list {
        println!("{v}");
    }
    let slice = &list[..];
    for (i, s) in slice.iter().enumerate() {
        println!("{s}{}", "~".repeat(i + 1));
    }
}

pub fn tuple_expressions() {
    let t0 = ();
    let t1 = (true,);
    let t2 = ('O', 2u8);
    let (name, title) = ("Ferris", "King Crab".to_string());
    let t3 = (name, "the", title);
    println!("t0: {t0:?};");
    println!("t1: {t1:?};");
    println!("t1: {t2:?};");
    println!("t1: {t3:?};");
}

pub fn range_demo() {
    let r1 = 1..=5;
    r1.for_each(|v| print!("{v}, "));
}
