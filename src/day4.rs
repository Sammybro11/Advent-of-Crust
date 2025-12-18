use std::fs;

struct Neighbours {
    row: isize,
    col: isize,
    idx: usize,
    offsets: [(isize, isize); 8],
    max_r: isize,
    max_c: isize,
}

impl Neighbours {
    fn construct(r: usize, c:usize, rows: usize, cols: usize) -> Self {
        Self {
            row: r as isize,
            col: c as isize,
            idx: 0,
            offsets: [
                (-1,-1), (-1, 0), (-1, 1),
                ( 0,-1),          ( 0, 1),
                ( 1,-1), ( 1, 0), ( 1, 1),
            ],
            max_r: rows as isize,
            max_c: cols as isize,
        }
    }
}

impl Iterator for Neighbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.offsets.len() {
            let (dr, dc) = self.offsets[self.idx];
            self.idx += 1;

            let nr = self.row + dr;
            let nc = self.col + dc;

            if nr >= 0 && nc >= 0 && nr < self.max_r && nc < self.max_c {
                return Some((nr as usize, nc as usize))
            }
        }
        None
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/input/day4.txt").unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut count: u32 = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    println!("{:?}", grid);
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    loop {
        let mut changed: u32 = 0;
        for i in 0..rows {
            // println!("Row: {}", i);
            for j in 0..cols {
                if grid[i][j] == '@' {
                    let it = Neighbours::construct(i, j, rows, cols);
                    let mut adj: u8 = 0;

                    for (nr, nc) in it {
                        if grid[nr][nc] == '@' {
                            adj += 1;
                        }
                    }
                    if adj < 4 {
                        grid[i][j] = 'x';
                        changed += 1;
                    }
                }
            }
        }
        if changed == 0 {
            break;
        }else{
            count += changed;
        }
    }
    println!("{:?}", grid);
    println!("{}", count);

}
