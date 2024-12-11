use std::collections::HashMap;

advent_of_code::solution!(11);

fn solve(input: &str, blinks: u32) -> u64 {
    let initial_stones = input
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let mut stone_counts: HashMap<u64, u64> = HashMap::new();
    for stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_stone_counts: HashMap<u64, u64> = HashMap::new();
        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                *new_stone_counts.entry(1).or_insert(0) += count;
            } else {
                let digits = stone.to_string();
                if digits.len() % 2 == 0 {
                    let mid = digits.len() / 2;
                    let left = digits[..mid].parse::<u64>().unwrap();
                    let right = digits[mid..].parse::<u64>().unwrap();
                    *new_stone_counts.entry(left).or_insert(0) += count;
                    *new_stone_counts.entry(right).or_insert(0) += count;
                } else {
                    *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
        stone_counts = new_stone_counts;
    }

    stone_counts.values().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = solve(input, 25);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = solve(input, 75);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
