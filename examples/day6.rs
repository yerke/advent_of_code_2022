use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day6.txt")?;

    for (i, window) in input.as_bytes().windows(4).enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(window);
        if set.len() == 4 {
            println!("part 1: {}", i + 4);
            break;
        }
    }

    for (i, window) in input.as_bytes().windows(14).enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(window);
        if set.len() == 14 {
            println!("part 2: {}", i + 14);
            break;
        }
    }

    Ok(())
}
