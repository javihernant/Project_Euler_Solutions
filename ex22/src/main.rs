use std::fs::File;
use std::io::prelude::*;

fn main() {
let mut names = File::open("names.txt").expect("File not found");
    let mut content = String::new();
    names.read_to_string(&mut content).expect("Sth went wrong");

   println!("{}", compute(content))

    }

    fn compute(w: String) -> usize {
        let mut score = 0;
        let v = w.split('"').collect::<Vec<&str>>();

           let mut v2:Vec<_>= v.iter().filter(|x| !x.is_empty() && x.chars().all(|l|l.is_alphabetic() )).collect();

        v2.sort();

        for (i, wo) in v2.iter().enumerate(){
        score += (i+1)*value_wo(&wo);
        }
        score
}

fn value_wo(name: &str) -> usize {
    name.chars().fold(0, |acc, x| acc+ value_le(x.to_lowercase().to_string()) as usize)
}

fn value_le(l: String) -> u32 {
    match l.as_ref() {
        "a"=> 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e"=> 5,
        "f" => 6,
        "g"=> 7,
        "h"=> 8,
        "i" => 9,
        "j"=> 10,
        "k" => 11,
        "l"=> 12,
        "m" => 13,
        "n" => 14,
        "o"=> 15,
        "p"=> 16,
        "q"=> 17,
        "r" => 18,
        "s"=> 19,
        "t" => 20,
        "u"=> 21,
        "v"=> 22,
        "w"=> 23,
        "x" => 24,
        "y"=> 25,
        "z"=> 26,
        _ => panic!()
    }
}
