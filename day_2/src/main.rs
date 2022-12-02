use itertools::Itertools;
use std::fs;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SHAPE_SCORE: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("A", 1);
        m.insert("B", 2);
        m.insert("C", 3);
        m.insert("X", 1);
        m.insert("Y", 2);
        m.insert("Z", 3);
        m
    };
}

const ARRAY: [usize; 3] = [1, 2, 3];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let score = solve_part1(&input);
    println!("Part 1: {}", score);

    let score = solve_part2(&input);
    println!("Part 2: {}", score);
}

fn solve_part1(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let (opponent_choice, own_choice) = line
            .split(' ')
            .map(|x| SHAPE_SCORE[x])
            .collect_tuple()
            .unwrap();

        if own_choice == opponent_choice {
            // draw
            score += own_choice + 3;
        } else if own_choice == ARRAY[opponent_choice % 3] {
            // win
            score += own_choice + 6;
        } else {
            // loss
            score += own_choice;
        }
    }
    score
}

fn solve_part2(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        let (opponent_choice, round_end) = line.split(' ').collect_tuple().unwrap();
        let opponent_score = SHAPE_SCORE[opponent_choice];
        match round_end {
            "X" => {
                // Lose
                let v = ARRAY[(opponent_score as i32 - 2) as usize % 3];
                score += v;
            }
            "Y" => {
                // Draw - Match opponent choice
                score += opponent_score + 3;
            }
            "Z" => {
                // Win
                let v = ARRAY[opponent_score % 3];
                score += v + 6;
            }
            _ => panic!(""),
        }
    }
    score
}
