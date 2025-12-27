use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/input/day5.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    // let mut count: u64 = 0;
    let mut reading: bool = true;

    for line in input.lines() {
        if line == "" {
            reading = false;
            ranges.sort_by_key(|&(start, _)| start);
            merge_ranges(&mut ranges);
            continue;
        }

        if reading {
            // println!("Reading: {}", line);
            let range: Vec<u64> = line.split('-')
                                    .map(|s| s.parse::<u64>().unwrap())
                                    .collect();
            // println!("Reading: {}, {}",range[0], range[1]);
            ranges.push((range[0], range[1]));
        }
        // else {

            // Part 1
            // // println!("Checking: {}", line);
            // match binary_search(&ranges, line.parse::<u64>().unwrap()) {
            //     Some(value) => {
            //         println!("Found {}", value);
            //         count += 1;
            //     },
            //     None => {},
            // }
        // }
    }

    // Part 2
    let count: u64 = ranges.iter().map(|&(start, end)| end - start + 1).sum();
    println!("Total Count: {}", count);
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    if ranges.is_empty() {
        return;
    }

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    *ranges = merged;
}

// fn binary_search(ranges: &Vec<(u64, u64)>, value: u64) -> Option<u64> {
//     let mut left = 0;
//     let mut right = ranges.len();
//
//     while left < right {
//         let mid = left + (right - left) / 2;
//         let (start, end) = ranges[mid];
//
//         if value < start {
//             right = mid;
//         } else if value > end {
//             left = mid + 1;
//         } else {
//             return Some(value);
//         }
//     }
//
//     None
// }
