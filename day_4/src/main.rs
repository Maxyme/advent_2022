use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum_full_overlap = solve_part_1(&input);
    println!("Part 1: {}", sum_full_overlap);

    let sum_partial_overlap = solve_part_2(&input);
    println!("Part 2: {}", sum_partial_overlap);
}

fn overlaps(min: usize, max: usize, lower_bound: usize, upper_bound: usize) -> bool {
    min >= lower_bound && min <= upper_bound || max >= lower_bound && max <= upper_bound
}

fn solve_part_2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (first, second) = line.split(',').collect_tuple().unwrap();
        let (first_min, first_max) = first
            .split('-')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let (second_min, second_max) = second
            .split('-')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let first_overlap_second = overlaps(first_min, first_max, second_min, second_max);
        let second_overlap_first = overlaps(second_min, second_max, first_min, first_max);

        if first_overlap_second || second_overlap_first {
            sum += 1;
        }
    }
    sum
}

fn solve_part_1(input: &str) -> usize {
    let mut sum_fully_cover = 0;
    for line in input.lines() {
        let (first, second) = line.split(',').collect_tuple().unwrap();
        let (first_min, first_max) = first
            .split('-')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let (second_min, second_max) = second
            .split('-')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let second_inside_1 = first_min <= second_min && first_max >= second_max;
        let first_inside_2 = first_min >= second_min && first_max <= second_max;
        if second_inside_1 || first_inside_2 {
            sum_fully_cover += 1;
        }
    }
    sum_fully_cover
}
