extern crate num_bigint ;

use num_bigint::{BigUint, ToBigUint};




fn main() {
println!("{}", compute(100).to_string().chars().fold(0, |acc, x| acc + x.to_digit(10).unwrap()));
}


fn compute(n:u64) -> BigUint {
    (1..n).fold(1.to_biguint().unwrap(), |acc, x| acc * x)
}