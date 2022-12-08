use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day8.txt")?;

    let mut tree_heights: Vec<i32> = Vec::new();
    let mut width: i32 = 0;
    for l in input.lines() {
        width = l.as_bytes().len() as i32;
        for c in l.bytes() {
            tree_heights.push((c - b'0') as i32);
        }
    }
    let height: i32 = (tree_heights.len()/width as usize) as i32;
    let neighbor_height = |x, y| -> i32 {
        if y < 0 || y >= height || x < 0 || x >= width {
            return -1; // so if the neighbor's height, it would still be visible
        }
        tree_heights[(x + y * width) as usize]
    };

    let mut part_1 = 0;
    for y in 1..height-1 {
        for x in 1..width-1 {
            let h = neighbor_height(x, y);
            'outer: for (d_x, d_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut x1 = x;
                let mut y1 = y;
                loop {
                    x1 += d_x;
                    y1 += d_y;
                    let h1 = neighbor_height(x1, y1);
                    if h1 == -1 {
                        part_1 += 1;
                        break 'outer;
                    }
                    if h1 >= h {
                        // found blocking tree
                        continue 'outer;
                    }
                }
            }
        }
    }

    part_1 += 2 * width + 2 * height - 4; // outer trees

    // part 2
    let mut max_product = 0;
    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut product = 1;
            let h = neighbor_height(x, y);
            for (d_x, d_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut x1 = x;
                let mut y1 = y;
                let mut viewing_distance = 0;
                loop {
                    x1 += d_x;
                    y1 += d_y;
                    let h1 = neighbor_height(x1, y1);
                    if h1 == -1 {
                        break;
                    }
                    viewing_distance += 1;
                    if h1 >= h {
                        break;
                    }
                }
                product *= viewing_distance;
            }

            if max_product < product {
                max_product = product;
            }
        }
    }

    println!("part 1: {part_1}");
    println!("part 2: {max_product}");

    Ok(())
}
