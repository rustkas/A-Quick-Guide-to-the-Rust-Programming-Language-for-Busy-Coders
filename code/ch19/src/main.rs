use ch19::{closure_demo_1, fib};

fn main() {
    closure_demo_1();

    let mut fib= fib();
    let fib_result = fib();

    println!("Fibonnachy result = {fib_result}");



}
