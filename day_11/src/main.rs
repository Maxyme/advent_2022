use std::collections::HashMap;
use std::fs;


#[derive(Clone, Debug, Default)]
struct Monkey {
    items: Vec<usize>,
    operator: char,
    num: Option<usize>,
    test_divider: usize,
    next: [usize; 2], // 0 - next monkey if test is true, 1 - false
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Parse monkeys
    let monkeys = parse_lines(&input);

    // Solve loop
    let monkey_business_level = solve(&monkeys, 20, 3);
    println!("Part 1: {monkey_business_level}");

    let monkey_business_level = solve(&monkeys, 10000, 1);
    println!("Part 2: {monkey_business_level}");
}

fn solve(monkeys: &HashMap<usize, Monkey>, rounds: usize, worry_divider: usize) -> usize {
    let mut monkeys = monkeys.clone();
    let num_monkeys = monkeys.len();

    let mut inspect_counts = HashMap::<usize, usize>::new();

    // Since all divisors are prime numbers, we can multiply them together to find the common modulo
    let common_modulo: usize = monkeys.values().map(|m| m.test_divider).product();

    for _ in 0..rounds {
        for i in 0..num_monkeys {
            let monkey = monkeys.get(&i).unwrap().clone();
            for item in &monkey.items {
                let mut worry_level = match monkey.num {
                    // Todo use enum or closure here
                    Some(x) => {
                        if monkey.operator == '+' {
                            item + x
                        } else {
                            item * x
                        }
                    }
                    None => {
                        if monkey.operator == '+' {
                            item + item
                        } else {
                            item * item
                        }
                    }
                };
                worry_level /= worry_divider;
                worry_level %= common_modulo;

                let next_monkey_index = {
                    if worry_level % monkey.test_divider == 0 {
                        monkey.next[0]
                    } else {
                        monkey.next[1]
                    }
                };

                let next_monkey = monkeys.get_mut(&next_monkey_index).unwrap();
                next_monkey.items.push(worry_level);
            }
            let current_count = inspect_counts.entry(i).or_default();
            *current_count += &monkey.items.len();

            let monkey = monkeys.get_mut(&i).unwrap();
            monkey.items = vec![];
        }
    }

    let mut values: Vec<&usize> = inspect_counts.values().collect();
    values.sort();

    values[values.len() - 1] * values[values.len() - 2]
}

fn parse_lines(input: &str) -> HashMap<usize, Monkey> {
    let mut monkeys = HashMap::<usize, Monkey>::new();
    let mut current_monkey = Monkey::default();
    let mut counter = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        match line.trim().split(' ').next().unwrap() {
            "Monkey" => {
                counter = line
                    .strip_prefix("Monkey ")
                    .unwrap()
                    .strip_suffix(':')
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            "Starting" => {
                let items_line = line.strip_prefix("  Starting items: ").unwrap();
                let x: Vec<&str> = items_line.split(',').collect();
                current_monkey.items = x.iter().map(|x| x.trim().parse().unwrap()).collect();
            }
            "Operation:" => {
                let operation_line = line.strip_prefix("  Operation: new = old ").unwrap();
                current_monkey.operator = operation_line.chars().next().unwrap();
                let value = operation_line
                    .strip_prefix(current_monkey.operator)
                    .unwrap()
                    .trim();
                if value != "old" {
                    current_monkey.num = Some(value.parse().unwrap());
                }
            }
            "Test:" => {
                let test_line = line.strip_prefix("  Test: divisible by ").unwrap();
                current_monkey.test_divider = test_line.parse().unwrap();
            }
            "If" => {
                if line.trim().starts_with("If true") {
                    let true_next = line.strip_prefix("    If true: throw to monkey ").unwrap();
                    current_monkey.next[0] = true_next.parse().unwrap();
                } else if line.trim().starts_with("If false") {
                    let false_next = line.strip_prefix("    If false: throw to monkey ").unwrap();
                    current_monkey.next[1] = false_next.parse().unwrap();
                    // Last line, insert monkey
                    monkeys.insert(counter, current_monkey.clone());
                }
            }
            _ => {}
        }
    }
    monkeys
}
