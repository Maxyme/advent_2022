use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1: rope of 2 segments
    let visited = solve_1(&input);
    println!("Part 1: {}", visited.len());

    // Part 2: rope of 9 segments
    let visited = solve_2(&input);
    println!("Part 2: {}", visited.len());
}

/// Check visited for 9 segments
fn solve_2(input: &str) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    // Insert origin
    visited.insert((0, 0));

    let mut positions: [(i32, i32); 10] = [(0, 0); 10];
    for line in input.lines() {
        let (dir, count) = line.split(' ').collect_tuple().unwrap();
        let count = count.parse().unwrap();
        for _ in 0..count {
            let head_position = &mut positions[0];
            // Update head position
            match dir {
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => panic!(),
            }

            for index in 1..positions.len() {
                let (above, segment_pos) = (positions[index - 1], &mut positions[index]);
                let distance = (above.0 - segment_pos.0, above.1 - segment_pos.1);
                if distance.0.abs() <= 1 && distance.1.abs() <= 1 {
                    // break early if no propagation
                    break;
                } else {
                    // Update segment position
                    segment_pos.0 += distance.0.signum();
                    segment_pos.1 += distance.1.signum();
                    if index == 9 {
                        visited.insert(*segment_pos);
                    }
                }
            }
        }
    }
    visited
}

/// Check visited for 2 segments
fn solve_1(input: &str) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();

    // Insert origin
    visited.insert((0, 0));
    let mut head_position = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);

    for line in input.lines() {
        let (dir, count) = line.split(' ').collect_tuple().unwrap();
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            let previous_head_position = head_position;
            // Update head position
            match dir {
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => panic!(),
            }

            // Then update tail position if euclidean distance more than 1 away
            let distance_t_h = (
                head_position.0 - tail_position.0,
                head_position.1 - tail_position.1,
            );
            if distance_t_h.0.abs() > 1 || distance_t_h.1.abs() > 1 {
                // move tail
                tail_position = previous_head_position;
                visited.insert(tail_position);
            }
        }
    }
    visited
}
