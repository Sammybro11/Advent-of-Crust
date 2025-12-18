mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day: u32 = 4;
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => println!("Big day for the unemployed.")
    }
}
