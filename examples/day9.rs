use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day9.txt")?;

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut tail1_past_positions: HashSet<(i32, i32)> = HashSet::new();
    tail1_past_positions.insert((rope[1].0, rope[1].1));
    let mut tail9_past_positions: HashSet<(i32, i32)> = HashSet::new();
    tail9_past_positions.insert((rope[9].0, rope[9].1));

    for l in input.lines() {
        let parts: Vec<&str> = l.split(' ').collect();
        let direction = parts[0];
        let number_of_steps: usize = parts[1].parse()?;

        for _ in 0..number_of_steps {
            match direction {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!(),
            }

            for t in 0..9 {
                if too_far(rope[t].0, rope[t].1, rope[t+1].0, rope[t+1].1) {
                    let new_tail = bring_closer(rope[t].0, rope[t].1, rope[t+1].0, rope[t+1].1);
                    rope[t+1].0 = new_tail.0;
                    rope[t+1].1 = new_tail.1;
                }
            }

            tail1_past_positions.insert((rope[1].0, rope[1].1));
            tail9_past_positions.insert((rope[9].0, rope[9].1));
        }
    }

    println!("part 1: {}", tail1_past_positions.len());
    println!("part 2: {}", tail9_past_positions.len());

    Ok(())
}

fn too_far(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    (x1 - x2).abs().max((y1 - y2).abs()) > 1
}

fn bring_closer(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let d_x = (head_x - tail_x).abs().min(1) * (head_x - tail_x).signum();
    let d_y = (head_y - tail_y).abs().min(1) * (head_y - tail_y).signum();
    (tail_x + d_x, tail_y + d_y)
}
