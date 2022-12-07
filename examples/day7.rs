use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Default, Debug)]
struct Dir {
    child_dir_idxs: HashMap<String, usize>,
    files: Vec<usize>,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("./data/day7.txt")?;

    let mut all_dirs: Vec<Dir> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let root = Dir::default();
    all_dirs.push(root);
    stack.push(0); // root index

    for l in input.lines() {
        if l.starts_with("$ cd ") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let new_dir_name = parts[2];

            if new_dir_name == "/" {
                stack.clear();
                stack.push(0);
            } else if new_dir_name == ".." {
                stack.pop();
            } else {
                let cur_dir = &mut all_dirs[*stack.last().unwrap()];
                let dir_idx = cur_dir.child_dir_idxs[new_dir_name];
                stack.push(dir_idx);
            }
        } else if l == "$ ls" {
            // we can ignore it
        } else if l.starts_with("dir ") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let child_dir_name = parts[1];

            let cur_dir = &all_dirs[*stack.last().unwrap()];
            if cur_dir.child_dir_idxs.contains_key(child_dir_name) {
                panic!("We are traversing the directory again");
            }

            let new_dir_idx = all_dirs.len();
            all_dirs.push(Dir::default());

            let cur_dir = &mut all_dirs[*stack.last().unwrap()];
            cur_dir.child_dir_idxs.insert(child_dir_name.to_owned(), new_dir_idx);
        } else {
            // size filename
            let parts: Vec<&str> = l.split_whitespace().collect();
            let size = parts[0].parse::<usize>()?;
            let cur_dir = &mut all_dirs[*stack.last().unwrap()];
            cur_dir.files.push(size);
        }
    }

    let mut dir_sizes = vec![0; all_dirs.len()];
    for i in (0..all_dirs.len()).rev() {
        let dir = &all_dirs[i];
        let filesize_sum: usize = dir.files.iter().sum();
        let child_dir_filesize_sum: usize = dir.child_dir_idxs.iter().map(|(_, &idx)| {
            dir_sizes[idx]
        }).sum();
        dir_sizes[i] = filesize_sum + child_dir_filesize_sum;
    }

    let part1: usize = dir_sizes.iter().filter(|&&x| x <= 100_000).sum();
    println!("part 1: {part1}");

    dir_sizes.sort();
    let unused_space = 70_000_000 - dir_sizes.last().unwrap();
    let min_space_to_free = 30_000_000 - unused_space;

    let dir_size_to_free = dir_sizes.iter().find(|&&x| x >= min_space_to_free).unwrap();
    println!("part 2: {dir_size_to_free}");

    Ok(())
}
