use std::{
    process::{ExitCode, Termination},
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Debug)]
struct Outcome(i32);
#[derive(Debug)]
struct Fault;
impl Termination for Outcome {
    fn report(self) -> ExitCode {
        if self.0 == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}
fn main() -> Result<Outcome, Fault> {
    println!("Hello, Rust the Crab!");
    if let Ok(true) = SystemTime::now().duration_since(UNIX_EPOCH).and_then(|d| {
        if d.as_millis() % 2 == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }) {
        Ok(Outcome(0))
    } else {
        Err(Fault)
    }
}
