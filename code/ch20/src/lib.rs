pub fn simple_type_aliases() {
    type Vital = (f32, i16);
    let v: Vital = (37.5, 65);
    println!(
        "Body temp:\t{},
Heart rate:\t{}",
        v.0, v.1
    );
}

pub fn generic_type_aliases() {
    type IOResult<T> = Result<T, &'static str>;
    type I32Result = IOResult<i32>;
    let r1: I32Result = Ok(42);
    let r2: I32Result = Err("File error");
    println!("r1 = {r1:?}; r2 = {r2:?}");
}

type BoolFn = fn(bool) -> bool;
fn do_boolean(f: BoolFn, a: bool) -> bool {
    f(a)
}
pub fn function_type_aliases() {
    let f1 = |_| true;
    let f2 = |_| false;
    let f3 = |a: bool| -> bool { a };
    let f4 = |a: bool| -> bool { !a };
    for f in [f1, f2, f3, f4] {
        for b in [true, false] {
            let x = do_boolean(f, b);
            print!("{x}\t");
        }
        println!();
    }
}
