extern crate core;

use std::fs::read_to_string;

struct Maze {
    maze: Vec<u8>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    sand_source_x: usize,
    sand_source_y: usize,
}

impl Maze {
    const VALID_MOVES: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];

    fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize, sand_source_x: usize, sand_source_y: usize) -> Self {
        let maze: Vec<u8> = vec![0; (max_x - min_x + 1) * (max_y - min_y + 1)];
        Maze {
            maze,
            min_x,
            max_x,
            min_y,
            max_y,
            sand_source_x,
            sand_source_y,
        }
    }

    fn translate_coordinates(&self, x: usize, y: usize) -> usize {
        (x - self.min_x) + (y - self.min_y) * (self.max_x - self.min_x + 1)
    }

    fn fill(&mut self, x: usize, y: usize, v: u8) {
        let idx = self.translate_coordinates(x, y);
        self.maze[idx] = v;
    }

    fn add_sand(&mut self) -> bool {
        let sand_source_idx = self.translate_coordinates(self.sand_source_x, self.sand_source_y);
        if self.maze[sand_source_idx] != 0 {
            panic!("Unexpectedly sand source is occupied");
        }

        let (mut x, mut y) = (self.sand_source_x, self.sand_source_y);
        'outer: loop {
            for valid_move in Self::VALID_MOVES {
                let x1 = (x as isize + valid_move.0) as usize;
                let y1 = (y as isize + valid_move.1) as usize;
                if !self.are_valid_coordinates(x1, y1) {
                    return false; // going to fall to abyss
                }

                let idx = self.translate_coordinates(x1, y1);
                if self.maze[idx] != 0 {
                    continue;
                }

                x = x1;
                y = y1;
                continue 'outer;
            }

            // couldn't find a valid move down
            break;
        }

        self.fill(x, y, 2);
        true
    }

    fn are_valid_coordinates(&self, x: usize, y: usize) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    fn print_maze(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let idx = self.translate_coordinates(x, y);
                let v = match self.maze[idx] {
                    0 => '.',
                    1 => '#',
                    2 => 'o',
                    _ => panic!("unexpected value in maze"),
                };
                print!("{v}");
            }
            println!();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day14.txt")?;

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (usize::MAX, usize::MIN, usize::MAX, usize::MIN);
    for l in input.lines() {
        let path: Vec<(usize, usize)> = l.split(" -> ").map(|p| -> (usize, usize) {
            let coord: Vec<usize> = p.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            let (x, y) = (coord[0], coord[1]);
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            (coord[0], coord[1])
        }).collect();
        paths.push(path);
    }

    // adjust for sand source
    let (sand_source_x, sand_source_y) = (500, 0);
    min_x = min_x.min(sand_source_x);
    max_x = max_x.max(sand_source_x);
    min_y = min_y.min(sand_source_y);
    max_y = max_y.max(sand_source_y);

    let mut maze = Maze::new(min_x, max_x, min_y, max_y, sand_source_x, sand_source_y);
    for path in &paths {
        for couple in path.windows(2) {
            let (x1, y1) = couple[0];
            let (x2, y2) = couple[1];
            if x1 == x2 {
                let start_y = y1.min(y2);
                let end_y = y1.max(y2);
                for y in start_y..=end_y {
                    maze.fill(x1, y, 1);
                }
            } else {
                // y1 == y2
                let start_x = x1.min(x2);
                let end_x = x1.max(x2);
                for x in start_x..=end_x {
                    maze.fill(x, y1, 1);
                }
            }
        }
    }

    let mut steps = 0;
    while maze.add_sand() {
        steps += 1;
        // println!("After step {steps}");
        // maze.print_maze();
    }

    println!("part 1: {steps}");

    Ok(())
}
