use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Element {
    Int(u8),
    List(List)
}

#[derive(Debug, Clone)]
struct List {
    elements: Vec<Box<Element>>
}

impl List {
    fn new() -> List {
        List {
            elements: Vec::new()
        }
    }

    fn new_with_one_element(e: Element) -> List {
        let mut elements: Vec<Box<Element>> = Vec::new();
        elements.push(Box::new(e));

        List {
            elements
        }
    }
}

impl PartialEq<Self> for List {
    fn eq(&self, other: &Self) -> bool {
        let decision = is_in_right_order(&self, other);
        match decision {
            Decision::Correct => false,
            Decision::Incorrect => false,
            Decision::Inconclusive => true,
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let decision = is_in_right_order(&self, other);
        match decision {
            Decision::Correct => Some(Ordering::Less),
            Decision::Incorrect => Some(Ordering::Greater),
            Decision::Inconclusive => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Decision {
    Correct,
    Incorrect,
    Inconclusive,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day13.txt")?;

    let mut pairs: Vec<(List, List)> = Vec::new();
    let mut left_list: Option<List> = None;
    let mut is_left = true;
    for l in input.lines() {
        if l == "" {
            continue;
        }

        let mut stack: Vec<List> = Vec::new();
        stack.push(List::new());
        let mut bytes_iter = l.bytes().peekable();
        loop {
            if let Some(b) = bytes_iter.next() {
                if b == b'[' {
                    let list = List::new();
                    stack.push(list);
                } else if b == b']' {
                    let list = stack.pop().unwrap();
                    let parent_list = stack.last_mut().unwrap();
                    parent_list.elements.push(Box::new(Element::List(list)));
                } else if b == b',' {
                    // noop
                } else {
                    // digit
                    let mut number = b - b'0';
                    while bytes_iter.peek().is_some() {
                        let next_byte = *bytes_iter.peek().unwrap();
                        if next_byte >= b'0' && next_byte <= b'9' {
                            let b = bytes_iter.next().unwrap();
                            number = number * 10 + b - b'0';
                        } else {
                            // next byte was not a digit
                            break;
                        }
                    }

                    // we consumed the whole integer
                    let list = stack.last_mut().unwrap();
                    list.elements.push(Box::new(Element::Int(number)));
                }
            } else {
                break;
            }
        }

        assert_eq!(stack.len(), 1);
        let mut list = stack.pop().unwrap();
        assert_eq!(list.elements.len(), 1);
        let element = *list.elements.pop().unwrap();

        let list: List;
        match element {
            Element::List(l) => list = l,
            Element::Int(_) => panic!(),
        }

        if is_left {
            left_list = Some(list);
            is_left = false;
        } else {
            pairs.push((left_list.take().unwrap(), list));
            is_left = true;
        }
    }

    // part 1
    let mut part1 = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        let decision = is_in_right_order(left, right);
        if decision == Decision::Inconclusive {
            panic!();
        }
        if decision == Decision::Correct {
            part1 += i + 1;
        }
    }

    println!("part 1: {part1}");

    // part 2
    let mut packets: Vec<List> = Vec::new();
    for (left, right) in pairs {
        packets.push(left);
        packets.push(right);
    }

    let divider_2 = List::new_with_one_element(Element::List(List::new_with_one_element(Element::Int(2))));
    let divider_6 = List::new_with_one_element(Element::List(List::new_with_one_element(Element::Int(6))));
    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let divider_2_idx = packets.iter().position(|l| l == &divider_2).unwrap() + 1;
    let divider_6_idx = packets.iter().position(|l| l == &divider_6).unwrap() + 1;
    println!("part 2: {}", divider_2_idx * divider_6_idx);

    Ok(())
}

fn pretty_print(l: &List) {
    pretty_print_internal(l);
    println!();
}

fn pretty_print_internal(l: &List) {
    print!("[");
    for e in l.elements.iter() {
        match &**e {
            Element::List(l) => pretty_print_internal(&l),
            Element::Int(n) => print!("{},", n),
        }
    }
    print!("]");
}

fn is_in_right_order(left: &List, right: &List) -> Decision {
    let mut left_idx = 0;
    let mut right_idx = 0;

    loop {
        let left_el = left.elements.get(left_idx);
        let right_el = right.elements.get(right_idx);

        match (left_el, right_el) {
            (None, None) => return Decision::Inconclusive, // Is it supposed to happen at all?
            (Some(_), None) => return Decision::Incorrect,
            (None, Some(_)) => return Decision::Correct,
            (Some(left_el), Some(right_el)) => {
                let left_el = *left_el.clone();
                let right_el = *right_el.clone();
                match (left_el, right_el) {
                    (Element::Int(left_i), Element::Int(right_i)) => {
                        if left_i < right_i {
                            return Decision::Correct;
                        } else if left_i > right_i {
                            return Decision::Incorrect;
                        }
                        // left_i == right_i, do nothing
                    },
                    (Element::List(left_l), Element::List(right_l)) => {
                        let decision = is_in_right_order(&left_l, &right_l);
                        if decision == Decision::Correct || decision == Decision::Incorrect {
                            return decision
                        }
                        // if inconclusive, do nothing
                    },
                    (Element::List(left_l), Element::Int(right_i)) => {
                        let right_l = List::new_with_one_element(Element::Int(right_i));
                        let decision = is_in_right_order(&left_l, &right_l);
                        if decision == Decision::Correct || decision == Decision::Incorrect {
                            return decision
                        }
                        // if inconclusive, do nothing
                    },
                    (Element::Int(left_i), Element::List(right_l)) => {
                        let left_l = List::new_with_one_element(Element::Int(left_i));
                        let decision = is_in_right_order(&left_l, &right_l);
                        if decision == Decision::Correct || decision == Decision::Incorrect {
                            return decision
                        }
                        // if inconclusive, do nothing
                    }
                }
            }
        }

        left_idx += 1;
        right_idx += 1;
    }
}
