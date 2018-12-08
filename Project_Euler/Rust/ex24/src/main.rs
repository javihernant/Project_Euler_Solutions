fn main() {
    // println!("{:?}", compute(1000000));


    compute(1000000);
}

fn compute(limit: u32) -> () {
    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    vec.reverse();
    //limit-1 times, first permutation not counted.
    for rep in 1..limit {
        for i in 1..vec.len() {
            if let Some(min) = vec.clone().iter().take(i)cd .min() {
                let prev = vec[i];
                let mut add_vec: Vec<u32> = vec.iter().take(i + 1).filter(|&x| x != min).map(|&x| x.clone()).collect();
                vec[i] = *min;
                add_vec.sort();
                add_vec.reverse();

                (0..add_vec.len()).for_each(|x| vec[x] = add_vec[x]);
                break;
            } else { continue; }

        }
       // println!("{:?}", vec);
    }
    vec.reverse();
    println!("{:?}", vec);
}
