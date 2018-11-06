fn main() {
    println!("{}", compute().to_string())
}

fn compute() -> u32 {

//1 Jan 1900 (mon)
//1 Jan 1901
 let mut count = 0;
    for y in 1901..(2000+1){
        let days_until_month: [u32; 13] = [
            0,
            31,
            if is_leap(y) {29}else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31 //not needed
        ];

        for i in 0..12 {
            let extra_days: u32 = days_until_month.iter().take(i+1).sum();
        if (days_until_jan_year(y) + extra_days) % 7 == 6 {
            count = count + 1;
        }
        }
    }


count
    }

fn days_until_jan_year(y: u32) -> u32 {
    let mut days = 0;

    for year in 1900..y {
        days = days + days_of_year(year);
    }
    days

}

fn days_of_year(y: u32) -> u32{
    if is_leap(y) {
        366
    }else {
        365
    }

}

fn is_leap(y:u32) -> bool{

    if y%100 == 0 {
        return false;
    }

    if y%400 == 0 {
        return true;
    }

    if y % 4 == 0{
        return true;
    }
    false
}



