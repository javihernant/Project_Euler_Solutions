fn main() {
    println! ("{}",compute(1000));
    //debug
  //  compute(1000);
}

fn compute(max:u64) ->u64 {
   (1..max+1)
        .map(to_word)
        .map(|w| w.chars().filter(|&w|w!=' ' && w!='-').count() as u64)
        .sum()

    //debug
 /*  let v: Vec<String> = (1..max+1)
        .map(to_word)
        .collect();
    println!("{:?}", v) */

}

fn to_word(n: u64) -> String {
    if n<1000 {
        return under_one_thousand(n);
    }
    "one thousand".to_string()
}

fn under_one_thousand(n: u64) -> String {
    if n < 10 {
        return under_ten(n)
    }

    if n < 20 {
        return under_twenty(n)
    }

    if n < 100 {
        return under_one_hundred(n)
    }

    format!("{} hundred{} {}",
            under_ten(n / 100),
            if under_one_thousand(n % 100) != "" { " and" } else { "" }
            , under_one_thousand(n % 100))
}

fn under_ten(n: u64) -> String {
    let num= match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!(),
    };
    num.to_owned()
}

    fn under_twenty(n:u64) -> String {
        let num = match n {
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!(),
        };
        num.to_owned()
    }

    fn under_one_hundred(n : u64) -> String {
        let num = match n/10{
            0|1 => panic!(),
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => panic!(),
        };
        format!("{} {}", num, under_ten(n%10) )
    }
