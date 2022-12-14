use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1: What is the sum of the indices of the pairs that are already in order.
    let mut sum = 0;

    let mut line_1: String = "".to_string();
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        } else if index % 3 == 0 {
            line_1 = line.to_string();
        } else {
            match compare_pairs(&line_1, line) {
                Some(v) => {
                    if v {
                        println!("{}, {} ok", &line_1, line);
                        sum += (index / 3) + 1
                    }
                }
                None => continue,
            }
        }
    }
    println!("Part 1: {sum}");

    // Part 2, put all the lines in order
    let mut all_pairs: Vec<String> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    all_pairs.push("[[2]]".to_string());
    all_pairs.push("[[6]]".to_string());
    all_pairs.sort_by(|a, b| match compare_pairs(a, b) {
        Some(v) => {
            if v {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        None => Ordering::Equal,
    });

    let mut ind_1 = 0;
    let mut ind_2 = 0;
    for (ind, v) in all_pairs.iter().enumerate() {
        if v == &"[[2]]".to_string() {
            ind_1 = ind + 1;
        }
        if v == &"[[6]]".to_string() {
            ind_2 = ind + 1;
        }
    }
    println!("Part 2: {}", ind_1 * ind_2);
}

fn extract(value: &str) -> Vec<String> {
    // Remove outside brackets
    let new_value = &value[1..value.len() - 1];
    let mut opening_brackets: VecDeque<usize> = VecDeque::new();
    let mut components = Vec::new();
    let mut current_value = "".to_string();
    for (index, char) in new_value.chars().enumerate() {
        if char == ',' && !current_value.is_empty() {
            components.push(current_value.to_string());
            current_value = "".to_string();
        } else if char == '[' {
            opening_brackets.push_back(index);
        } else if char == ']' {
            let previous = opening_brackets.pop_back().unwrap();
            if opening_brackets.is_empty() {
                let sub = &new_value[previous..=index];
                components.push(sub.to_string());
            }
        } else if opening_brackets.is_empty() && char != ',' {
            current_value.push(char);
        }
    }
    if !current_value.is_empty() {
        components.push(current_value);
    }
    components
}
fn compare_pairs(left: &str, right: &str) -> Option<bool> {
    // If both values are integers
    if right == "[]" && left != "[]" {
        return Some(false);
    }
    if right != "[]" && left == "[]" {
        return Some(true);
    }
    if let (Ok(l), Ok(r)) = (left.parse::<usize>(), right.parse::<usize>()) {
        if l == r {
            None
        } else {
            Some(l < r)
        }
    } else if left.contains('[') && right.contains('[') {
        let left_components = extract(left);
        let right_components = extract(right);

        for (index, left_sub) in left_components.iter().enumerate() {
            match right_components.get(index) {
                Some(right_sub) => {
                    let decision = compare_pairs(left_sub, right_sub);
                    match decision {
                        Some(v) => return Some(v),
                        None => continue,
                    }
                }
                // Right list ran out of items
                None => return Some(false),
            }
        }
        if left_components.len() < right_components.len() {
            // left list ran out of objects first, in order
            return Some(true);
        }
        // lists are the same length and no comparison makes a decision about the order
        return None;
    } else {
        // Exactly one value is an integer
        if left.len() < right.len() {
            return compare_pairs(&format!("[{left}]"), right);
        } else {
            return compare_pairs(left, &format!("[{right}]"));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{compare_pairs, extract};
    #[test]
    fn test_extract_0() {
        let input_1 = "[1,1,3]";
        let result = extract(input_1);
        assert_eq!(result, vec!["1", "1", "3"]);
    }

    #[test]
    fn test_extract_1() {
        let input_1 = "[[1]]";
        let result = extract(input_1);
        assert_eq!(result, vec!["[1]"]);
    }

    #[test]
    fn test_extract_2() {
        let input_1 = "[[1],[2,3,4]]";
        let result = extract(input_1);
        assert_eq!(result, vec!["[1]", "[2,3,4]"]);
    }

    #[test]
    fn test_extract_3() {
        let input_1 = "[[[]]]";
        let result = extract(input_1);
        assert_eq!(result, vec!["[[]]"]);
    }

    #[test]
    fn test_extract_4() {
        let input_1 = "[[4,4],4,4]";
        let result = extract(input_1);
        assert_eq!(result, vec!["[4,4]", "4", "4"]);
    }

    #[test]
    fn test_extract_5() {
        let input_1 = "[4,[5],6]";
        let result = extract(input_1);
        assert_eq!(result, vec!["4", "[5]", "6"]);
    }

    #[test]
    fn test_extract_6() {
        let input_1 = "[2,[3,[4,[5,6,7]]]]";
        let result = extract(input_1);
        assert_eq!(result, vec!["2", "[3,[4,[5,6,7]]]"]);
    }

    #[test]
    fn test_extract_7() {
        let input_1 = "[[1],[2,3,4]]";
        let result = extract(input_1);
        assert_eq!(result, vec!["[1]", "[2,3,4]"]);
    }

    #[test]
    fn test_extract_8() {
        let input_1 = "[]";
        let result = extract(input_1);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_extract_9() {
        let input_1 = "[3]";
        let result = extract(input_1);
        assert_eq!(result, vec!["3"]);
    }

    #[test]
    fn test_compare_0() {
        let input_1 = "10";
        let input_2 = "11";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_compare_1() {
        let input_1 = "10";
        let input_2 = "10";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_compare_2() {
        let input_1 = "[1,1,3,1,1]";
        let input_2 = "[1,1,5,1,1]";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_compare_3() {
        let input_1 = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let input_2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, Some(false));
    }

    #[test]
    fn test_compare_4() {
        let input_1 = "[]";
        let input_2 = "[3]";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_compare_5() {
        let input_1 = "[[10]]";
        let input_2 = "[[[1,[5,4,0,10,0]]],[8],[[[],5,10],9,9,5,8],[[[10,6,1],6,8,5,8],3,10,[[],2,[],2,3],[[6],[6,6,3],[10,0,0,7,6]]],[[2],[],[[],4,1]]]";
        let result = compare_pairs(input_1, input_2);
        assert_eq!(result, Some(false));
    }

    // #[test]
    // fn test_1() {
    //     let input_1 = "[1,10,3,1,1]";
    //     let input_2 = "[1,10,5,1,1]";
    //     let result = compare_pairs(input_1, input_2);
    //     assert_eq!(result, true);
    // }
    //
    // #[test]
    // fn test_3() {
    //     let input_1 = "[[4,4],4,4]";
    //     let input_2 = "[[4,4],4,4,4]";
    //     let result = compare_pairs(input_1, input_2);
    //     assert_eq!(result, true);
    // }
    //
    // #[test]
    // fn test_4() {
    //     let input_1 = "[[4,4],4,4,4]";
    //     let input_2 = "[[4,4],4,4]";
    //     let result = compare_pairs(input_1, input_2);
    //     assert_eq!(result, false);
    // }
}
