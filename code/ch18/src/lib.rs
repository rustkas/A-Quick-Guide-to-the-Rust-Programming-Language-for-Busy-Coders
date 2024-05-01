const A: i32 = 32;
const B: i64 = 64;
pub const C: i64 = cfun();

const fn cfun() -> i64 {
    A as i64 + B + 48
}

async fn afun() -> i32 {
    42
}
pub async fn async_function_demo() ->i32 {
    let x = afun();
    let y = x.await;
    assert_eq!(42, y);
    y
}
