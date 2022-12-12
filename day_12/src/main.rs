use std::cmp::min;
use ndarray::{s, Array, Array1, Array2};
use std::collections::BinaryHeap;
use std::fs;


static ASCII_LOWER: [char; 28] = [
    'S', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'E'
];


fn main() {
    // Part 1: find the path with the lowest total risk
    let input = fs::read_to_string("input.txt").unwrap();

    // Read into 2d array
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();
    let mut grid: Array2<char> = Array::default((num_lines, line_len));
    for (index, values) in input.lines().enumerate() {
        let values: Array1<char> = values.chars().collect();
        let mut row_at_index = grid.slice_mut(s![index, ..]);
        row_at_index.assign(&values);
    }

    // Get start and end positions
    let start = grid.indexed_iter().find(|(_, &x)| x == 'S').unwrap().0;
    let end = grid.indexed_iter().find(|(_, &x)| x == 'E').unwrap().0;

    let destinations_costs = get_destination_costs(&grid, start);
    let destination_min_cost = destinations_costs.get(end).unwrap();
    println!("Part 1: {destination_min_cost}");

    let mut min_cost = usize::MAX;
    let start_positions = grid.indexed_iter().filter(|(_, &x)| x == 'a');
    for (pos, _) in start_positions {
        let destinations_costs = get_destination_costs(&grid, pos);
        let destination_min_cost = destinations_costs.get(end).unwrap();
        min_cost = min(min_cost, *destination_min_cost);
    }
    println!("Part 2: {min_cost}");
}

fn can_visit(from: char, to: char) -> bool {
    fn get_position(c: char) -> usize {
        ASCII_LOWER.iter().position(|&x| x == c).unwrap()
    }

    // return true if to == from + 1 or to is lower than from
    get_position(from) + 1 >= get_position(to)
}


fn get_destination_costs(grid: &Array2<char>, start: (usize, usize)) -> Array2<usize> {
    // Create a queue with start already added
    let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    heap.push(start);

    // Create a map of minimum destination costs for each node (coord) in the array from 0,0
    fn function() -> usize {
        usize::MAX
    }
    let mut destinations_costs: Array2<usize> = Array2::from_shape_simple_fn(grid.dim(), function);

    // Set src known cost of 0
    destinations_costs[start] = 0;

    let mut visited: Array2<bool> = Array2::default(grid.dim());
    while let Some(node) = heap.pop() {
        // Get node value
        let node_letter = grid[node];

        // Get the min cost to get to that node from the start
        let node_dist_cost = destinations_costs[node];

        // Check each possible pixel neighbor
        if node.0 > 0 {
            let neigh = (node.0 - 1, node.1);
            if can_visit(node_letter, grid[neigh]) {
                visit(&mut heap, &mut destinations_costs, &mut visited, node_dist_cost, neigh);
            }
        }
        if node.0 < grid.nrows() - 1 {
            let neigh = (node.0 + 1, node.1);
            if can_visit(node_letter, grid[neigh]) {
                visit(&mut heap, &mut destinations_costs, &mut visited, node_dist_cost, neigh);
                visited[neigh] = true;
            }
        }
        if node.1 > 0 {
            let neigh = (node.0, node.1 - 1);
            if can_visit(node_letter, grid[neigh]) {
                visit(&mut heap, &mut destinations_costs, &mut visited, node_dist_cost, neigh);
            }
        }
        if node.1 < grid.ncols() - 1 {
            let neigh = (node.0, node.1 + 1);
            if can_visit(node_letter, grid[neigh]) {
                visit(&mut heap, &mut destinations_costs, &mut visited, node_dist_cost, neigh);
            }
        }
    }
    destinations_costs
}

fn visit(heap: &mut BinaryHeap<(usize, usize)>, destinations_costs: &mut Array2<usize>, visited: &mut Array2<bool>, node_dist_cost: usize, neigh: (usize, usize)) {
    let dist_cost = node_dist_cost + 1;
    let previously_computed_cost = destinations_costs.get_mut(neigh).unwrap();
    if dist_cost < *previously_computed_cost {
        // New cost is smaller, update the previous known cost
        *previously_computed_cost = dist_cost;
        heap.push(neigh);
    } else if !visited[neigh] {
        // If not visited then add to queue
        heap.push(neigh);
    }

}
