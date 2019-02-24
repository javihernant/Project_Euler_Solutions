use std::env;
use std::fs;


fn main() {
   /*    let num = 25;
       let va = ((num as f64).sqrt() as usize + 1);

       for b in 2..va {
           if num % b == 0 {
               if b*b != num {
                   println!("1");
                   println!("{},{}", b,num/b);
               }
           }

       }*/



   println!("Hello, world! {}", compute());
}

fn compute() -> usize {
    let mut vec = Vec::new();
    let limit = 28123;
    let mut v_solution = vec![0; limit + 1];

    for i in 1..=limit {
        let mut sum = 1;
        for b in 2..((i as f64).sqrt() as usize + 1) {
            if i % b == 0 {
                if b * b != i {
                    sum += i / b + b;
                }else {
                    sum += b;
                }
            }
        }
        if sum > i { vec.push(i) }
    }

    for abnum in &vec {
        for abnum2 in &vec {
            let abnum_sum = abnum + abnum2;
            if abnum_sum > limit {
                break;
            }

            v_solution[abnum_sum] = 1;
        }
    }
    // println!("{:?}", v_solution);


    let mut solution: usize = 0;
    for (i, _el) in v_solution.iter().enumerate() {
        if (v_solution)[i] == 0 {
            solution += i;
        }
    }
    solution
}

