use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day5.txt")?;

    let mut stacks_part1: Vec<VecDeque<u8>> = Vec::new();
    let mut stacks_part2: Vec<VecDeque<u8>> = Vec::new();

    let mut parse_instructions = false;
    for l in input.lines() {
        if !parse_instructions {
            if l.trim() == "" {
                parse_instructions = true;
                for i in 0..stacks_part1.len() {
                    stacks_part2.push(stacks_part1[i].clone());
                }
                continue;
            }

            let mut remaining = l;
            let mut offset = 0;
            loop {
                let p = remaining.find('[');
                if p.is_none() {
                    break;
                }

                let p = p.unwrap();
                offset += p / 4;
                let letter = remaining.as_bytes()[p+1];
                while stacks_part1.len() < offset + 1 {
                    stacks_part1.push(VecDeque::new());
                }
                stacks_part1[offset].push_front(letter);

                if remaining.len() < p + 4 {
                    break;
                }
                remaining = &remaining[p+4..];
                offset += 1;
            }
        } else {
            // parse instructions
            let parts: Vec<&str> = l.split_whitespace().collect();
            let number_of_blocks: usize = parts[1].parse()?;
            let from: usize = parts[3].parse::<usize>()? - 1;
            let to: usize = parts[5].parse::<usize>()? - 1;

            // part 1
            for _ in 0..number_of_blocks {
                let block = stacks_part1[from].pop_back().unwrap();
                stacks_part1[to].push_back(block);
            }

            // part 2
            let mut temp = VecDeque::new();
            for _ in 0..number_of_blocks {
                temp.push_front(stacks_part2[from].pop_back().unwrap());
            }
            stacks_part2[to].append(&mut temp);
        }
    }

    println!("Part 1");
    for i in 0..stacks_part1.len() {
        print!("{}", stacks_part1[i][stacks_part1[i].len() - 1] as char);
    }
    println!();

    println!("Part 2");
    for i in 0..stacks_part2.len() {
        print!("{}", stacks_part2[i][stacks_part2[i].len() - 1] as char);
    }
    println!();

    Ok(())
}
