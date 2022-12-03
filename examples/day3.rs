#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day3.txt")?;

    // part 1
    let mut part1_total = 0;
    input
        .split("\n")
        .for_each(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            let left_set: HashSet<char> = left.chars().collect();
            let right_set: HashSet<char> = right.chars().collect();
            let intersection: HashSet<&char> = left_set.intersection(&right_set).collect();
            let score = get_score(intersection);
            part1_total += score;
        }
        );

    // part 2
    let chunks = input
        .split("\n")
        .array_chunks::<3>();

    let mut part2_total = 0;
    for chunk in chunks {
        let a_set: HashSet<char> = chunk[0].chars().collect();
        let b_set: HashSet<char> = chunk[1].chars().collect();
        let c_set: HashSet<char> = chunk[2].chars().collect();
        let intersection: HashSet<char> = a_set.intersection(&b_set).copied().collect();
        let intersection: HashSet<&char> = intersection.intersection(&c_set).collect();
        let score = get_score(intersection);
        part2_total += score;
    }

    println!("{part1_total}");
    println!("{part2_total}");

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
