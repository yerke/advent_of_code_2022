use std::collections::HashMap;
use std::fs::read_to_string;

const PLURAL_FILLER: &str = "; tunnels lead to valves ";
const SINGULAR_FILLER: &str = "; tunnel leads to valve ";
const MAX_TIME_PART1: i32 = 30;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day16.txt")?;

    let mut start_node: &str = "AA";
    let mut graph: HashMap<&str, (u8, Vec<&str>)> = HashMap::new();
    for l in input.lines() {
        let l = l.strip_prefix("Valve ").unwrap();
        let parts: Vec<&str> = l.split(" has flow rate=").collect();
        let node = parts[0];
        let remaining = parts[1];


        let filler = if remaining.find(PLURAL_FILLER).is_some() {
            PLURAL_FILLER
        } else {
            SINGULAR_FILLER
        };
        let parts: Vec<&str> = remaining.split(filler).collect();
        let flow = parts[0].parse::<u8>().unwrap();
        let neighbors: Vec<&str> = parts[1].trim().split(", ").collect();

        graph.insert(node, (flow, neighbors));
    }

    let mut nodes: Vec<&str> = graph.keys().map(|&n| n).collect();
    let number_of_nodes = nodes.len();
    nodes.sort();

    let mut weights: Vec<i32> = vec![i32::MAX; number_of_nodes * number_of_nodes];
    for (&node, (_, neighbors)) in &graph {
        let node_idx = nodes.iter().position(|&n| n == node).unwrap();

        for &neighbor in neighbors {
            let neighbor_idx = nodes.iter().position(|&n| n == neighbor).unwrap();
            weights[node_idx * number_of_nodes + neighbor_idx] = 1;
        }
    }

    let (distance, next) = floyd_warshall(&weights, number_of_nodes);
    pretty_print_distance(&distance, &nodes);
    pretty_print_path(&next, number_of_nodes, 0, 7, &nodes);

    // part 1
    // TODO: Add flows array
    let mut nodes_with_flow: Vec<&str> = graph.iter().filter_map(|(n, (flow, _))| if *flow > 0 { Some(*n) } else { None }).collect();

    // we need to add start node
    if nodes_with_flow.iter().position(|n| *n == start_node).is_none() {
        nodes_with_flow.push(start_node);
    }
    nodes_with_flow.sort();
    println!("{:?}", &nodes_with_flow);
    let start_node_with_flow_idx = nodes_with_flow.iter().position(|n| *n == start_node).unwrap();

    let number_of_nodes_with_flow = nodes_with_flow.len();
    let mut weights_with_flow: Vec<i32> = vec![i32::MAX; number_of_nodes_with_flow * number_of_nodes_with_flow];
    let mut flows_of_nodes_with_flow: Vec<u8> = vec![0; number_of_nodes_with_flow];

    for (from_idx, &from) in nodes_with_flow.iter().enumerate() {
        let original_from_idx = nodes.iter().position(|n| *n == from).unwrap();

        flows_of_nodes_with_flow[from_idx] = graph.get(from).unwrap().0;

        for (to_idx, &to) in nodes_with_flow.iter().enumerate() {
            let original_to_idx = nodes.iter().position(|n| *n == to).unwrap();
            weights_with_flow[from_idx * number_of_nodes_with_flow + to_idx] = distance[original_from_idx * number_of_nodes + original_to_idx];

            if from_idx != to_idx {
                // We are adding 1 to account for time to open the valve
                weights_with_flow[from_idx * number_of_nodes_with_flow + to_idx] += 1;
            }
        }
    }

    println!("{:?}", &flows_of_nodes_with_flow);

    pretty_print_distance(&weights_with_flow, &nodes_with_flow);

    let (max_total_value_part1, max_ordering_part1) = depth_first_search(&weights_with_flow, &flows_of_nodes_with_flow, number_of_nodes_with_flow, start_node_with_flow_idx)?;

    println!("part 1: {max_total_value_part1}");

    println!("max_ordering_part1");
    let mut running_time = 0;
    let mut running_total = 0;
    let mut value_per_minute = 0;
    let mut previous = None;
    for node_idx in max_ordering_part1 {
        if previous.is_some() {
            let time = weights_with_flow[previous.unwrap() * number_of_nodes_with_flow + node_idx];
            running_time += time;
            running_total += time * value_per_minute;
            value_per_minute += flows_of_nodes_with_flow[node_idx] as i32;
            println!("{} -> {}: (time: {time}, running time: {running_time}, running total: {running_total}, new value per minute: {value_per_minute})", nodes_with_flow[previous.unwrap()], nodes_with_flow[node_idx]);
        }
        previous = Some(node_idx);
    }
    let remaining_time = MAX_TIME_PART1 - running_time;
    println!("after that: (time: {remaining_time}, running total: {})", remaining_time * value_per_minute + running_total);

    Ok(())
}

fn floyd_warshall(weights: &Vec<i32>, number_of_nodes: usize) -> (Vec<i32>, Vec<Option<u32>>) {
    let mut distance: Vec<i32> = vec![i32::MAX; number_of_nodes * number_of_nodes];
    let mut next: Vec<Option<u32>> = vec![None; number_of_nodes * number_of_nodes];

    for i in 0..number_of_nodes {
        for j in 0..number_of_nodes {
            distance[i * number_of_nodes + j] = weights[i * number_of_nodes + j];
            if weights[i * number_of_nodes + j] != i32::MAX {
                next[i * number_of_nodes + j] = Some(j as u32);
            }
        }
    }

    for i in 0..number_of_nodes {
        distance[i * number_of_nodes + i] = 0;
        next[i * number_of_nodes + i] = Some(i as u32);
    }

    for k in 0..number_of_nodes {
        for i in 0..number_of_nodes {
            for j in 0..number_of_nodes {
                if distance[i * number_of_nodes + j] > distance[i * number_of_nodes + k].saturating_add(distance[k * number_of_nodes + j]) {
                    distance[i * number_of_nodes + j] = distance[i * number_of_nodes + k] + distance[k * number_of_nodes + j];
                    next[i * number_of_nodes + j] = next[i * number_of_nodes + k];
                }
            }
        }
    }

    return (distance, next)
}

fn pretty_print_distance(distance: &Vec<i32>, nodes: &Vec<&str>) {
    for from in 0..nodes.len() {
        for to in 0..nodes.len() {
            println!("from {} to {}: {}", nodes[from], nodes[to], distance[from * nodes.len() + to]);
        }
    }
}

fn pretty_print_path(next: &Vec<Option<u32>>, number_of_nodes: usize, from: usize, to: usize, nodes: &Vec<&str>) {
    let path = path(next, number_of_nodes, from, to);
    print!("path from {from} to {to}: ");
    for node_idx in path {
        print!("{}, ", nodes[node_idx]);
    }
    println!();
}

fn path(next: &Vec<Option<u32>>, number_of_nodes: usize, from: usize, to: usize) -> Vec<usize> {
    let mut path = Vec::new();
    if next[from * number_of_nodes + to].is_none() {
        return path;
    }

    let mut current = from;
    path.push(current);
    while current != to {
        current = next[current * number_of_nodes + to].unwrap() as usize;
        path.push(current);
    }

    return path;
}

fn depth_first_search(weights: &Vec<i32>, flows: &Vec<u8>, number_of_nodes: usize, start_idx: usize) -> anyhow::Result<(u32, Vec<usize>)> {
    // current node index, time after going to the new node, value per minute, total value up to till current moment
    let mut stack: Vec<(usize, i32, u32, u32)> = vec![(0, 0, 0, 0); number_of_nodes];
    stack[0] = (start_idx, 0, 0, 0);
    let (max_total_value, max_ordering) = check_next_level(weights, flows, number_of_nodes, &mut stack, 1)?;

    Ok((max_total_value, max_ordering))
}

fn check_next_level(weights: &Vec<i32>, flows: &Vec<u8>, number_of_nodes: usize, stack: &mut Vec<(usize, i32, u32, u32)>, cur_depth: usize) -> anyhow::Result<(u32, Vec<usize>)> {
    let (previous_node, previous_time, previous_value_per_minute, previous_total_value) = stack[cur_depth - 1];

    // Check the total value if we stay in this place and don't move
    let total_value_if_we_stay_here = (MAX_TIME_PART1 - previous_time) as u32 * previous_value_per_minute + previous_total_value;
    let mut max_total_value = total_value_if_we_stay_here;
    let mut max_ordering: Vec<usize> = stack[0..cur_depth].iter().map(|(n, _, _, _)| *n).collect();

    if cur_depth == number_of_nodes {
        return Ok((max_total_value, max_ordering));
    }

    for next_node in 0..number_of_nodes {
        let previous_nodes = &stack[0..cur_depth];
        if previous_nodes.iter().position(|(n, _, _, _)| *n == next_node).is_some() {
            // already visited that node
            continue;
        }

        let extra_time = weights[previous_node * number_of_nodes + next_node];
        if extra_time == i32::MAX {
            // no path there to previous node to next node
            continue;
        }

        if previous_time + extra_time > MAX_TIME_PART1 {
            // we don't have time to visit next node
            continue;
        }

        let extra_value = previous_value_per_minute * extra_time as u32;
        let new_total_value = previous_total_value + extra_value;
        let new_time = previous_time + extra_time;
        let new_value_per_minute = previous_value_per_minute + flows[next_node] as u32;

        stack[cur_depth] = (next_node, new_time, new_value_per_minute, new_total_value);
        let (total_value, ordering) = check_next_level(weights, flows, number_of_nodes, stack, cur_depth + 1)?;
        if total_value > max_total_value {
            max_total_value = total_value;
            max_ordering = ordering;
        }
    }

    Ok((max_total_value, max_ordering))
}
