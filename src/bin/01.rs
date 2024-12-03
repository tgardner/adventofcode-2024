use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut lhs, mut rhs): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let mut nums = l.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    lhs.sort();
    rhs.sort();

    (lhs, rhs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (lhs, rhs) = parse_input(input);

    let result = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lhs, rhs) = parse_input(input);
    let occurrences = rhs.iter().fold(HashMap::new(), |mut acc, &r| {
        *acc.entry(r).or_insert(0) += 1;
        acc
    });

    let result = lhs
        .iter()
        .map(|l| l * occurrences.get(l).unwrap_or(&0))
        .sum::<i32>();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
