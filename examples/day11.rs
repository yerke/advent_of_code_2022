use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Clone)]
struct Monkey<F>
where
    F: Fn(usize) -> usize
{
    items: VecDeque<usize>,
    op: F,
    test_arg: usize,
    if_true_target_monkey: usize,
    if_false_target_monkey: usize,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day11.txt")?;

    let mut monkeys: Vec<Monkey<_>> = Vec::new();
    let mut lines = input.lines();
    loop {
        lines.next().unwrap(); // Monkey <number>

        let items = lines.next().unwrap();
        let items = items.strip_prefix("  Starting items: ").unwrap();
        let items: VecDeque<usize> = items.split(", ").map(|i| i.parse::<usize>().unwrap()).collect();

        let op = lines.next().unwrap();
        let op = op.strip_prefix("  Operation: new = old ").unwrap();
        let op_parts: Vec<&str> = op.split_whitespace().collect();
        let is_addition = op_parts[0] == "+"; // multiplication otherwise
        let is_second_arg_old = op_parts[1] == "old";
        let mut second_arg: usize = 0;
        if !is_second_arg_old {
            second_arg = op_parts[1].parse()?;
        }
        let op = move |x: usize| -> usize {
            if is_second_arg_old {
                if is_addition {
                    x + x
                } else {
                    x * x
                }
            } else {
                if is_addition {
                    x + second_arg
                } else {
                    x * second_arg
                }
            }
        };

        let test_arg = lines.next().unwrap();
        let test_arg = test_arg.strip_prefix("  Test: divisible by ").unwrap();
        let test_arg: usize = test_arg.parse()?;

        let if_true_target_monkey = lines.next().unwrap();
        let if_true_target_monkey = if_true_target_monkey.strip_prefix("    If true: throw to monkey ").unwrap();
        let if_true_target_monkey: usize = if_true_target_monkey.parse()?;

        let if_false_target_monkey = lines.next().unwrap();
        let if_false_target_monkey = if_false_target_monkey.strip_prefix("    If false: throw to monkey ").unwrap();
        let if_false_target_monkey: usize = if_false_target_monkey.parse()?;

        let monkey = Monkey {
            items,
            op,
            test_arg,
            if_true_target_monkey,
            if_false_target_monkey,
        };

        monkeys.push(monkey);

        if lines.next().is_none() {
            break;
        };
    }

    let monkeys_part2 = monkeys.clone();

    // part 1
    let mut inspections = vec![0; monkeys.len()];
    for round in 0..20 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let current_items: VecDeque<usize> = m.items.drain(0..).collect();
            let m = monkeys[i].clone();
            for &item in current_items.iter() {
                inspections[i] += 1;
                let item = (m.op)(item);
                let item = item / 3;
                let is_true = item % m.test_arg == 0;
                let target_monkey = if is_true { m.if_true_target_monkey } else { m.if_false_target_monkey };
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }

    inspections.sort();
    println!("part 1: {}", inspections.pop().unwrap() * inspections.pop().unwrap());

    // part 2
    let mut monkeys = monkeys_part2;
    let mut product_of_test_args = 1;
    for m in &monkeys {
        product_of_test_args *= m.test_arg;
    }
    let mut inspections = vec![0; monkeys.len()];
    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let current_items: VecDeque<usize> = m.items.drain(0..).collect();
            let m = monkeys[i].clone();
            for &item in current_items.iter() {
                inspections[i] += 1;
                let item = (m.op)(item);
                let item = item % product_of_test_args;
                let is_true = item % m.test_arg == 0;
                let target_monkey = if is_true { m.if_true_target_monkey } else { m.if_false_target_monkey };
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }

    inspections.sort();
    let arg1: i128 = inspections.pop().unwrap() as i128;
    let arg2: i128 = inspections.pop().unwrap() as i128;

    println!("part 2: {}", arg1 * arg2);

    Ok(())
}

impl<F> Monkey<F>
where
    F: Fn(usize) -> usize
{
    fn print_monkey_items(&self) {
        println!("{:?}", self.items);
    }
}
