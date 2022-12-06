use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let buffer = input.lines().next().unwrap();

    let diff_char_count = 4;
    let index = get_index(buffer, diff_char_count);
    println!("Part 1: {}", index + 1);

    let diff_char_count = 14;
    let index = get_index(buffer, diff_char_count);
    println!("Part 2: {}", index + 1);
}

fn get_index(buffer: &str, diff_char_count: usize) -> usize {
    let mut previous_chars: Vec<char> = Vec::new();

    // todo use windows
    let index: usize = {
        let mut found_index = 0;
        for (index, c) in buffer.chars().enumerate() {
            previous_chars.push(c);
            if previous_chars.len() < diff_char_count {
                continue;
            }
            // Check if at least 4 are unique
            let previous_chars = &previous_chars[index - (diff_char_count - 1)..=index];
            let mut seen = HashSet::new();
            for p in previous_chars {
                if seen.contains(p) {
                    break;
                } else {
                    seen.insert(p);
                }
            }
            found_index = index;
            if seen.len() == diff_char_count {
                break;
            }
        }
        found_index
    };
    index
}
