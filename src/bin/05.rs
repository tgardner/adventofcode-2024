use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

// Check if the update satisfies the ordering rules
fn is_valid_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    let update_positions: std::collections::HashMap<_, _> =
        update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (update_positions.get(&x), update_positions.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }

    true
}

// Find the middle page of an update
fn find_middle(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

/// Reorder an update according to the rules
fn reorder_update(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();

    // Filter rules to only those relevant for the update
    let update_set: HashSet<_> = update.iter().cloned().collect();
    for &(x, y) in rules {
        if update_set.contains(&x) && update_set.contains(&y) {
            graph.entry(x).or_insert_with(HashSet::new).insert(y);
            *in_degree.entry(y).or_insert(0) += 1;
            in_degree.entry(x).or_insert(0);
        }
    }

    // Topological sort
    let mut sorted = Vec::new();
    let mut stack: Vec<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    while let Some(node) = stack.pop() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut sections = input.split("\n\n");
    let ordering_rules: Vec<(i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut nums = l.split("|").map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let updates = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (ordering_rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse_input(input);
    let mut total_middle_sum = 0;

    for update in updates {
        if is_valid_update(&update, &ordering_rules) {
            let middle = find_middle(&update);
            total_middle_sum += middle;
        }
    }
    Some(total_middle_sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse_input(input);
    let mut total_middle_sum = 0;

    for update in updates {
        if is_valid_update(&update, &ordering_rules) {
            continue; // Skip already valid updates
        } else {
            let corrected = reorder_update(&update, &ordering_rules);
            let middle = find_middle(&corrected);
            total_middle_sum += middle;
        }
    }
    Some(total_middle_sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
