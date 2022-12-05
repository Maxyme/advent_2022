use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let priority_sum = solve_part_1(&input);
    println!("Part 1: {}", priority_sum);

    let priority_sum = solve_part_2(&input);
    println!("Part 2: {}", priority_sum);
}

fn solve_part_2(input: &str) -> usize {
    let mut priority_sum = 0;

    let l: Vec<&str> = input.lines().collect();
    let groups = l.as_slice().chunks(3);

    // Todo use a functional method instead
    for group in groups {
        let mut a: HashSet<char> = HashSet::from_iter(group[0].chars());
        let set_b: HashSet<char> = HashSet::from_iter(group[1].chars());
        let set_c: HashSet<char> = HashSet::from_iter(group[2].chars());
        a.retain(|&k| set_b.contains(&k));
        a.retain(|&k| set_c.contains(&k));
        let common = a.iter().next().unwrap();
        let priority = priority_from_char(common);
        priority_sum += priority;
    }
    priority_sum
}

fn solve_part_1(input: &str) -> usize {
    let mut priority_sum = 0;
    for line in input.lines() {
        // Split in 2
        let (a, b) = line.split_at(line.len() / 2);

        // Find first common item and add priority  to sum
        let set_a: HashSet<char> = HashSet::from_iter(a.chars());
        let set_b: HashSet<char> = HashSet::from_iter(b.chars());
        let common = set_a.intersection(&set_b).next().unwrap();
        let priority = priority_from_char(common);

        priority_sum += priority
    }
    priority_sum
}

// Get priority using unicode code point
fn priority_from_char(common: &char) -> usize {
    if common.is_uppercase() {
        common.to_ascii_lowercase() as usize - 96 + 26
    } else {
        *common as usize - 96
    }
}
