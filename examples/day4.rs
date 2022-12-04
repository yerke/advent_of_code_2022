use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day4.txt")?;

    let mut part1_total = 0;
    let mut part2_total = 0;
    input
        .split("\n")
        .for_each(|l| {
            let parts: Vec<&str> = l.split(",").collect();
            let left = parts[0];
            let right = parts[1];
            let left_idxs = get_range(left);
            let right_idxs = get_range(right);

            if (left_idxs.0 <= right_idxs.0 && left_idxs.1 >= right_idxs.1)
                || (right_idxs.0 <= left_idxs.0 && right_idxs.1 >= left_idxs.1) {
                part1_total += 1;
            }

            if (left_idxs.0 <= right_idxs.1 && left_idxs.1 >= right_idxs.1)
                || (right_idxs.0 <= left_idxs.1 && right_idxs.1 >= left_idxs.1)  {
                part2_total += 1;
            }
        });

    println!("{part1_total}");
    println!("{part2_total}");

    Ok(())
}

fn get_range(s: &str) -> (u32, u32) {
    let parts: Vec<u32> = s
        .split("-")
        .map(|p| p.parse::<u32>().unwrap())
        .collect();

    (parts[0], parts[1])
}
