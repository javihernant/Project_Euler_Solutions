extern crate num_bigint;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    println!("Hello, world!, {}", f().unwrap());

}

fn f() -> Option<usize>{
    let mut a:BigUint = 1_u32.to_biguint().unwrap();
    let mut b:BigUint = 1_u32.to_biguint().unwrap();
  for i in 3..{
      let aux =a;
      a = b.clone();
      b=aux+b;
      let b_str = b.to_string();
      //println!("{}",b_str);
      if b_str.len() >= 1000 { return Some(i); }
  }

    None

    }



