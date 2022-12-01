use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut elves_calories = Vec::new();
    let mut curr_calories = 0;
    for line in input.lines() {
        if !line.is_empty() {
            curr_calories += line.parse::<usize>().unwrap();
        } else {
            // Next elf
            elves_calories.push(curr_calories);
            curr_calories = 0;
        }
    }
    println!("Part 1: {:?}", elves_calories.iter().max().unwrap());

    elves_calories.sort();
    let top_three: usize = elves_calories.iter().rev().take(3).sum();
    println!("Part 2: {:?}", top_three);
}
