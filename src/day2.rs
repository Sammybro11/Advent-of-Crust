use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let ranges = parse_ranges(&input);
    let mut total: u64 = 0;
    for (start, end) in ranges.iter() {
        println!("{} to {}", start, end);
        // Ok so what I am trying to do is this, 
        // get first half of start, since only twice repeating nums need to be added I need to make
        // it double, then I compare if the doubled is less than end if so then add it, then check
        // the half start again +1 and the double it again and so on till the double is less than
        // end and then we move to the next range...

        let s_start = start.to_string();
        let len_start = s_start.len();

        let mut half_val: u64 = match (len_start as u64) % 2 {
            1 => {
                let k = (len_start - 1 ) / 2;
                10u64.pow(k as u32).try_into().unwrap()
            },
            0 => {
                let half_len = len_start / 2;
                s_start[..half_len].parse().unwrap()
            },
            _ => {
                panic!("Mod 2 gave weird value");
            }
        };

        loop {
            let doubled_str = format!("{}{}", half_val, half_val);
            let doubled: u64 = doubled_str.parse().unwrap();

            if doubled < *start {
                half_val += 1;
                continue;
            }

            if doubled > *end {
                break;
            }
            println!("Adding {}", doubled);
            total += doubled;
            half_val += 1;
        }
    }
    println!("Total: {}", total)
}

fn parse_ranges(s: &str) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let mut part_iter = part.split('-');
        let start = part_iter.next().unwrap().trim().parse().unwrap();
        let end = part_iter.next().unwrap().trim().parse().unwrap();
        out.push((start, end));
    }
    out
}
