fn main() {
    println!("{}", compute(20));
}

fn compute(size:u64) -> u64{
    let mut n:u64 = 1;

    for i in 0..size{
        n *= size*2 - size + i + 1;
        n /= i+1;
    }
    n
}