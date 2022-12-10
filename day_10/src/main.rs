use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Create a stack of instructions
    let mut stack: VecDeque<Option<i32>> = VecDeque::new();
    for line in input.lines() {
        if line.starts_with("addx") {
            let value: i32 = line.split(' ').last().unwrap().parse().unwrap();
            stack.push_back(None);
            stack.push_back(Some(value));
        } else {
            // Add empty cycle
            stack.push_back(None);
        }
    }

    // Part 2. Find signal strength from instructions
    let mut x = 1;
    let mut signal_strength_sum = 0;
    let mut cycle = 1;
    let mut part_1_stack = stack.clone();
    loop {
        // Update signal strength
        if HashSet::<i32>::from_iter([20, 60, 100, 140, 180, 220]).contains(&cycle) {
            signal_strength_sum += cycle * x;
        }
        if cycle == 220 {
            break;
        }
        // Execute instruction
        if let Some(v) = part_1_stack.pop_front().unwrap() {
            x += v;
        }
        cycle += 1;
    }

    println!("Part 1: {signal_strength_sum}");

    // Part 2. simulate a crt drawing a single pixel per cycle
    let mut display = [['.'; 40]; 6];
    let mut x = 1;
    let mut cycle = 0_i32;
    while cycle < 240 {
        let row = cycle / 40;

        // Draw pixel
        let sprite_col = x % 40;
        let cycle_col = cycle % 40;
        let pixel = {
            if cycle_col >= (sprite_col - 1) && cycle_col <= (sprite_col + 1) {
                '#'
            } else {
                '.'
            }
        };
        display[row as usize][cycle_col as usize] = pixel;

        // Execute instruction (move sprite)
        if let Some(v) = stack.pop_front().unwrap() {
            x += v;
        }
        cycle += 1;
    }
    println!("Part 2: ");
    for line in display {
        println!("{line:?}");
    }
}
