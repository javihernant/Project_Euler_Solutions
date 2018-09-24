fn main() {

    let mut total = 0;
    for i in 1..10000{
        //compute(a) = b
        //compute(b) = a
        //compute(compute(b)) = b
        if compute(compute(i)) == i && compute(i)!= i{
            total = total + i;
            println!("{}", i )
        }
    }


println!("{}", total);
}

fn compute(n:u64) -> u64
{
    (1..n).filter(|x|n%x== 0).sum()
}
