use std::fs;
// File system module baby

pub fn solve() {
    let input = fs::read_to_string("src/input/day3.txt").unwrap();
    let mut joltage: u128 = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut highest;
        let mut index: u32 = 0;
        let len = line.len() as u32;
        let mut max: u128 = 0;

        let mut i: u32 = 0;

        let num_digits: u32 = 12;
        let mut count: u32 = 1;

        loop {
            match i {
                0 => {},
                _ => i = index + 1,
            }
            highest = 0;

            if count == num_digits + 1 {
                break;
            }

            loop {
                let num: u32 = chars[i as usize].to_digit(10).unwrap();
                if num > highest {
                    highest = num;
                    index = i;
                }
                if i >= (len - 1 - num_digits + count) {
                    break;
                }
                i += 1;
            }
            // println!("Adding {}, count: {}", highest, count);
            max += highest as u128 * 10u128.pow(num_digits - count);

            count += 1;
        }

        // loop {
        //     let num: u32 = digits[i as usize].to_digit(10).unwrap();
        //     if num > highest {
        //         highest = num;
        //         index = i;
        //     }
        //     if i >= (len - 2) {
        //         break;
        //     }
        //     i += 1;
        // }
        //
        // i = index + 1;
        //
        // loop {
        //     let num: u32 = digits[i as usize].to_digit(10).unwrap();
        //     if num > second_highest {
        //         second_highest = num;
        //         index = i;
        //     }
        //     if i >= (len - 1) {
        //         break;
        //     }
        //     i += 1;
        // }
        // let max = highest*10 + second_highest;
        joltage += max;
        println!("Bank Power {}", max);
    }
    println!("Joltage: {}", joltage)
}
