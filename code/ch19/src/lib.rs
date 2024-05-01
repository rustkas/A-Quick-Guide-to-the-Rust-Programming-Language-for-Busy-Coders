pub fn closure_demo_1() {
    let c1 = |x, y| x + y;
    let v1 = c1(1, 2);
    assert_eq!(v1, (|a, b| a + b)(1, 2));
    use std::f32::consts::PI;
    let c2 = |x: f32| -> f32 { x * PI };
    let v2 = c2(2.0);
    println!("value = {v2}");
}

pub fn unique_immutable_borrows() {
    let mut mr = false;
    let im2mr = &mut mr;
    let mut c = || *im2mr = true;
    // let can_i_borrow = &im2mr;
    c();
    let _refref = &im2mr;
    assert_eq!(*im2mr, true);
}

pub fn fib() -> impl FnMut() -> i32 {
    let (mut a, mut b) = (0, 1);
    move || {
        (a, b) = (b, a + b);
        b
    }
}

