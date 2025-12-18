use std::fs;
// File System Module

pub fn solve() {
    let input = fs::read_to_string("src/input/sample.txt").unwrap();
    // read_to_string gives output Result<String, std::io::Error>
    // Then unwrap basically does panic and show error message(default), could use .except() for 
    // original error message or could use match if we know program won't fail if func fails, or 
    // if it's part of functionality

    let mut pos: i32 = 50;
    let mut hits: i32 = 0;

    for (dir, mut value) in input.lines().filter_map(parse_instruction) {
        // Filter map just takes the values which aren't none...
        if value > 99 {
            hits += value / 100;
            value = value % 100;
        }
        let delta: i32 = match dir {
            'L' => -value,
            'R' => value,
            _ => panic!("Not a Valid Direction"),
        };

        let pos_new: i32 = (pos + delta).rem_euclid(100);

        if pos_new == 0 {
            hits += 1;
        }
        // if old val was not 0 and adding something pushed beyond edge then.
        else if pos != 0 && ((pos + delta > 99) || (pos + delta < 0)) {
            hits += 1;
        }
        pos = pos_new;
    }
    println!("Hits: {}", hits);

}

fn parse_instruction(s: &str) -> Option<(char, i32)> {
    // So I am assuming line could be empty and that shouldn't be a problem
    // So Option<> is used
    let mut chars = s.chars();

    // ? at the end just returns None if the value returned is None, so 
    // if next line is empty just return None so it gets filtered
    let direction = chars.next()?;
    let num_str: String = chars.collect();
    let value: i32 = num_str.parse().ok()?;

    Some((direction, value))
}
