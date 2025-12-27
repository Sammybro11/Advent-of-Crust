use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/input/day7.txt").unwrap();
    let first_line = input.lines().next().unwrap();
    let source_index = first_line.find('S').unwrap();
    let length = first_line.len();

    // Allocating both beams to be of length of line with all false initially
    let mut current_beam: Vec<bool> = vec![false; length];
    let mut next_beam: Vec<bool> = vec![false; length];
    let mut current_timelines: Vec<u64> = vec![0; length];
    let mut next_timelines: Vec<u64> = vec![0; length];
    let mut splits: u32 = 0;

    current_beam[source_index] = true;
    current_timelines[source_index] = 1;

    for line in input.lines() {
        // println!("Current Line: {:?}", current_timelines);
        for (idx, char) in line.char_indices() {
            if current_beam[idx] && char == '^' {
                splits += 1;
                if idx == 0 {
                    next_beam[idx + 1] = true;
                    next_timelines[idx + 1] += current_timelines[idx];
                } else if idx == length - 1 {
                    next_beam[idx - 1] = true;
                    next_timelines[idx - 1] += current_timelines[idx];
                } else {
                    next_beam[idx + 1] = true;
                    next_beam[idx - 1] = true;
                    next_timelines[idx + 1] += current_timelines[idx];
                    next_timelines[idx - 1] += current_timelines[idx];
                }
            } else if current_beam[idx] {
                next_beam[idx] = true;
                next_timelines[idx] += current_timelines[idx];
            }
        }
        current_beam = next_beam;
        current_timelines = next_timelines;
        next_beam = vec![false; length];
        next_timelines = vec![0; length];
    }
    println!("Total Splits: {}", splits);
    println!("Final Timelines: {:?} \n Total: {}", current_timelines, current_timelines.iter().sum::<u64>());
}
