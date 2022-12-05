use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day5.txt")?;

    let mut stacks = Vec::new();
    let mut parse_instructions = false;
    for l in input.lines() {
        if !parse_instructions {
            if l.trim() == "" {
                parse_instructions = true;
                continue;
            }

            let mut remaining = l;
            let mut offset = 0;
            loop {
                let p = remaining.find('[');
                dbg!(remaining);
                dbg!(p);
                if p.is_none() {
                    break;
                }

                let p = p.unwrap();
                offset += p / 4;
                dbg!(offset);
                let letter = remaining.as_bytes()[p+1];
                dbg!(letter);
                println!("{} as offset {}", letter as char, offset);
                while stacks.len() < offset + 1 {
                    stacks.push(VecDeque::new());
                }
                stacks[offset].push_front(letter);

                if remaining.len() < p + 4 {
                    break;
                }
                remaining = &remaining[p+4..];
                offset += 1;
            }
        } else {
            // parse instructions
            pretty_print(&stacks);
            let parts: Vec<&str> = l.split_whitespace().collect();
            let number_of_blocks: usize = parts[1].parse()?;
            let from: usize = parts[3].parse::<usize>()? - 1;
            let to: usize = parts[5].parse::<usize>()? - 1;
            println!("{number_of_blocks}, {from}, {to}");

            for _ in 0..number_of_blocks {
                let block = stacks[from].pop_back().unwrap();
                stacks[to].push_back(block);
            }
        }
    }

    for i in 0..stacks.len() {
        print!("{}", stacks[i][stacks[i].len() - 1] as char);
    }
    println!();

    Ok(())
}

fn pretty_print(stacks: &Vec<VecDeque<u8>>) {
    for i in 0..stacks.len() {
        let stack = &stacks[i];
        for j in 0..stack.len() {
            print!("{}", stack[j] as char);
        }
        println!();
    }
}
