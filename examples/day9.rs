use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day9.txt")?;

    let mut head_x: i32 = 0;
    let mut head_y: i32 = 0;
    let mut tail_x: i32 = 0;
    let mut tail_y: i32 = 0;
    let mut tail_past_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_past_positions.insert((tail_x, tail_y));

    for l in input.lines() {
        let parts: Vec<&str> = l.split(' ').collect();
        let direction = parts[0];
        let number_of_steps: usize = parts[1].parse()?;

        for i in 0..number_of_steps {
            match direction {
                "U" => head_y += 1,
                "D" => head_y -= 1,
                "L" => head_x -= 1,
                "R" => head_x += 1,
                _ => panic!(),
            }

            println!("head: {head_x}, {head_y}");

            if too_far(head_x, head_y, tail_x, tail_y) {
                let new_tail = bring_closer(head_x, head_y, tail_x, tail_y);
                tail_x = new_tail.0;
                tail_y = new_tail.1;
                // println!("moved tail to: {tail_x}, {tail_y}");
            }

            tail_past_positions.insert((tail_x, tail_y));
        }
    }

    // println!("part 1: {}", tail_past_positions.len());

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
