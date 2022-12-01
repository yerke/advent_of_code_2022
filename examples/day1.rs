use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day1.txt")?;

    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|lines| lines
            .split_whitespace()
            .fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
            )
        .collect();
    sums.sort();

    let len = sums.len();
    println!("max: {}", sums[len-1]);
    println!("max 3 sum: {}", sums[len-1] + sums[len-2] + sums[len-3]);

    Ok(())
}
