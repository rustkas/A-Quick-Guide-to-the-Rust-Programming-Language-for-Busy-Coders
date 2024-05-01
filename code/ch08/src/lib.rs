#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Qubit {
phi: f32,
psi: f32,
}

pub fn clone_demo() {
  let q1 = Qubit { phi: 0., psi: 3.14, };
  let q2 = q1.clone();
  println!("q1 = {q1:?}; q2 = {q2:?}");
  }