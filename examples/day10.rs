use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day10.txt")?;

    let mut x = 1;
    let mut t = 0;
    let mut part1_total = 0;

    let mut check_part1 = |t: i32, x: i32| {
        if (t - 20) % 40 == 0 {
            let product = t * x;
            part1_total += product;
            // println!("During {t} x = {x}, product = {product}, part1 total: {part1_total}");
        }
    };

    let mut crt: Vec<u8> = Vec::new();
    let mut draw_part2 = |_t: i32, x: i32| {
        let crt_idx = (crt.len() % 40) as i32;
        if crt_idx == x - 1 || crt_idx == x || crt_idx == x + 1 {
            crt.push(b'#');
        } else {
            crt.push(b'.');
        }
    };

    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0] == "noop" {
            t += 1;
            // during cycle
            check_part1(t, x);
            draw_part2(t, x);
        } else {
            // add
            t += 1;
            // during cycle
            check_part1(t, x);
            draw_part2(t, x);
            t += 1;
            // during cycle
            check_part1(t, x);
            draw_part2(t, x);
            let delta: i32 = parts[1].parse()?;
            x += delta;
        }
    }

    println!("part 1: {part1_total}");
    for i in 0..crt.len() {
        if i % 40 == 0 {
            println!();
        }
        print!("{}", crt[i] as char);
    }
    println!();

    Ok(())
}
