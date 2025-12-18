mod day1;
mod day2;
mod day3;

fn main() {
    let day: u32 = 3;
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        _ => println!("Big day for the unemployed.")
    }
}
