use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/input/day9.txt").unwrap();
    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in input.lines() {
        let mut coords = line.split(',');
        let x = coords.next().unwrap().parse::<i64>().unwrap();
        let y = coords.next().unwrap().parse::<i64>().unwrap();
        points.push((x, y));
    }

    let length = points.len();
    let mut areas: Vec<i64> = Vec::new();

    for i in 0..length {
        for j in (i + 1)..length {
            let dx = (points[i].0 - points[j].0 + 1).abs();
            let dy = (points[i].1 - points[j].1 + 1).abs();
            areas.push(dx * dy);
        }
    }

    let max = areas.iter().max().unwrap();
    println!("Max area: {}", max);
}
