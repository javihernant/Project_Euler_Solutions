extern crate num_bigint;
extern crate num_traits;
use num_bigint::BigUint;

fn main() {
let num = num_traits::pow(BigUint::from(2u8), 1000 as usize);
println!("{}", compute(num));
}
fn compute (n:BigUint) -> BigUint{
    n.to_string().chars().map(|c| c.to_digit(10).unwrap() ).sum()
}
