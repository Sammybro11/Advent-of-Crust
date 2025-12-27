mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;

fn main() {
    let day: u32 = 7;
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        7 => day7::solve(),
        _ => println!("Big day for the unemployed.")
    }
}
