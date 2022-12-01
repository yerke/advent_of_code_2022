use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day1.txt")?;
    let mut current_elf_sum = 0;
    let mut max_elf_sum = 0;
    for line in input.lines() {
        if line == "" {
            if current_elf_sum > max_elf_sum {
                max_elf_sum = current_elf_sum
            }
            current_elf_sum = 0;
            continue;
        }

        let calories = line.parse::<i32>()?;
        current_elf_sum += calories;
    }

    println!("max: {max_elf_sum}");

    Ok(())
}
