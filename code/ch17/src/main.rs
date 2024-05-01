use ch17::AnArray;

fn main() {
    let mut array1:AnArray<u8,3>;
    array1 = AnArray([&1, &2, &3]);
    println!("{array1:?}");

    array1 = AnArray([&1;3]);
    println!("{array1:?}");
}
