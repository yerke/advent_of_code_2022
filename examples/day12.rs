use std::collections::VecDeque;
use std::fs::read_to_string;

const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day12.txt")?;

    let mut heightmap: Vec<u8> = Vec::new();
    let mut width = 0;
    for l in input.lines() {
        width = l.len();
        heightmap.extend(l.bytes().map(|b| {
            if b == b'S' {
                255
            } else if b == b'E' {
                254
            } else {
                b - b'a'
            }
        }));
    }
    let width = width;
    let height = heightmap.len() / width;
    let start_idx = heightmap.iter().position(|&b| b == 255).unwrap();
    let end_idx = heightmap.iter().position(|&b| b == 254).unwrap();
    heightmap[start_idx] = 0;
    heightmap[end_idx] = b'z' - b'a';

    // part 1
    let part1 = depth_first_search_part1(&heightmap, width, height, start_idx, end_idx)?;
    println!("part 1: {part1}");

    // part 2
    let part2 = depth_first_search_part2(&heightmap, width, height, end_idx)?;
    println!("part 2: {part2}");

    Ok(())
}

fn depth_first_search_part1(heightmap: &Vec<u8>, width: usize, height: usize, start_idx: usize, end_idx: usize) -> anyhow::Result<usize> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; width * height];
    queue.push_back((0, start_idx % width, start_idx / width));
    visited[start_idx] = true;
    while queue.len() > 0 {
        let (cur_len, x, y) = queue.pop_front().unwrap();
        let current_idx = y * width + x;
        if current_idx == end_idx {
            return Ok(cur_len);
        }

        for delta in DELTAS {
            let x1 = x as i32 + delta.0;
            let y1 = y as i32 + delta.1;
            if !are_valid_coordinates(x1, y1, width, height) {
                continue;
            }

            let candidate_idx = y1 as usize * width + x1 as usize;

            if visited[candidate_idx] {
                continue;
            }

            if heightmap[candidate_idx] <= heightmap[current_idx] + 1 {
                visited[candidate_idx] = true;
                queue.push_back((cur_len + 1, x1 as usize, y1 as usize));
            }
        }
    }
    panic!("Failed to find end");
}

fn depth_first_search_part2(heightmap: &Vec<u8>, width: usize, height: usize, start_idx: usize) -> anyhow::Result<usize> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; width * height];
    queue.push_back((0, start_idx % width, start_idx / width));
    visited[start_idx] = true;
    while queue.len() > 0 {
        let (cur_len, x, y) = queue.pop_front().unwrap();
        let current_idx = y * width + x;
        if heightmap[current_idx] == 0 {
            return Ok(cur_len);
        }

        for delta in DELTAS {
            let x1 = x as i32 + delta.0;
            let y1 = y as i32 + delta.1;
            if !are_valid_coordinates(x1, y1, width, height) {
                continue;
            }

            let candidate_idx = y1 as usize * width + x1 as usize;

            if visited[candidate_idx] {
                continue;
            }

            if heightmap[current_idx] <= heightmap[candidate_idx] + 1 {
                visited[candidate_idx] = true;
                queue.push_back((cur_len + 1, x1 as usize, y1 as usize));
            }
        }
    }
    panic!("Failed to find end");
}

fn are_valid_coordinates(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
}
