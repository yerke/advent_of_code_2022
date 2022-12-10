use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day10.txt")?;

    let mut x = 1;
    let mut t = 0;
    let mut part1_total = 0;

    let mut check = |t: i32, x: i32| {
        if (t - 20) % 40 == 0 {
            let product = t * x;
            part1_total += product;
            println!("During {t} x = {x}, product = {product}, part1 total: {part1_total}");
        }
    };

    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0] == "noop" {
            t += 1;
            check(t, x);
        } else {
            // add
            t += 1;
            check(t, x);
            t += 1;
            check(t, x);
            let delta: i32 = parts[1].parse()?;
            x += delta;
        }
    }

    println!("{x}");
    println!("{t}");
    println!("part 1: {part1_total}");

    Ok(())
}
