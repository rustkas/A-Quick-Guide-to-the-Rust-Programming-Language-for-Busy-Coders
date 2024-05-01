use ch18::{async_function_demo, C};

#[tokio::main]
async fn main() {
    println!("{}", C);

    let function_demo_result = async_function_demo();

    let result = function_demo_result.await;
    println!("{}", result);
}
