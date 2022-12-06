extern crate core;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::str::Lines;

fn extract_command(input: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    }
    // Todo: clean this up!
    let caps = RE.captures(input).unwrap();
    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());
    let text3 = caps.get(3).map_or("", |m| m.as_str());
    (
        text1.parse::<usize>().unwrap(),
        text2.parse::<usize>().unwrap(),
        text3.parse::<usize>().unwrap(),
    )
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Create stacks
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if !line.chars().any(|x| x == '[') {
            break;
        }
        let vecs = line.chars().collect::<Vec<char>>();
        let s = vecs.as_slice();
        let crates = s.chunks(4);
        for (index, crate_letters) in crates.enumerate() {
            if crate_letters.iter().all(|x| x.is_whitespace()) {
                continue;
            }

            let stack = match stacks.get_mut(index) {
                Some(x) => x,
                None => {
                    // Add empty stacks
                    while stacks.len() < index + 1 {
                        stacks.push(VecDeque::new())
                    }
                    stacks.get_mut(index).unwrap()
                }
            };
            let letter = crate_letters.iter().find(|x| x.is_alphabetic()).unwrap();
            stack.push_front(*letter)
        }
    }

    // Read the empty line
    lines.next();

    // Execute the rest of the instructions
    let message = solve(&stacks, &lines, false);
    println!("Part 1: {}", message);

    let message = solve(&stacks, &lines, true);
    println!("Part 2: {}", message);
}

fn solve(stacks: &Vec<VecDeque<char>>, lines: &Lines, part_2: bool) -> String {
    let mut stacks = stacks.to_owned();
    for command in lines.clone() {
        let (count, from, to) = extract_command(command);

        let mut to_stack = stacks.get(to - 1).unwrap().clone();
        let mut from_stack = stacks.get(from - 1).unwrap().clone();
        if part_2 {
            // use a temporary stack
            let mut new_stack = VecDeque::new();
            for _ in 0..count {
                let elf_crate = from_stack.pop_back().unwrap();
                new_stack.push_front(elf_crate);
            }
            for s in new_stack {
                to_stack.push_back(s);
            }
        } else {
            for _ in 0..count {
                let elf_crate = from_stack.pop_back().unwrap();
                to_stack.push_back(elf_crate);
            }
        }

        // insert again in stacks
        stacks[to - 1] = to_stack;
        stacks[from - 1] = from_stack;
    }

    let message: String = stacks.iter().map(|x| x.back().unwrap()).collect();
    message
}
