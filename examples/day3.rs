use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day3.txt")?;

    let mut part1_total = 0;
    input
        .split("\n")
        .for_each(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            dbg!(left);
            dbg!(right);
            let left_set: HashSet<char> = left.chars().collect();
            let right_set: HashSet<char> = right.chars().collect();
            let intersection: HashSet<&char> = left_set.intersection(&right_set).collect();
            dbg!(&intersection);
            let score = get_score(intersection);
            dbg!(score);
            part1_total += score;
        }
        );

    println!("{part1_total}");

    Ok(())
}

fn get_score(s: HashSet<&char>) -> u32 {
    s.iter().fold(0, |acc, c| {
        match c {
            'a' ..= 'z' => acc + (**c as u32 - 'a' as u32 + 1),
            'A' ..= 'Z' => acc + (**c as u32 - 'A' as u32 + 27),
            _ => panic!("Unexpected char {c}"),
        }
    })
}
