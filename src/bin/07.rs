advent_of_code::solution!(7);

fn guess_operator(nums: &[usize], result: usize, operations: &[char]) -> bool {
    for op in operations {
        let val = match op {
            '+' => nums[0] + nums[1],
            '*' => nums[0] * nums[1],
            'c' => format!("{}{}", nums[0], nums[1]).parse::<usize>().unwrap(),
            _ => panic!("Invalid operator"),
        };

        match nums.len() {
            2 if val == result => return true,
            2 => continue,
            _ if guess_operator(&[&[val], &nums[2..]].concat(), result, operations) => return true,
            _ => continue,
        }
    }

    false
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let result: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|l| {
            let (res, operands) = l.split_once(": ").expect("parsed");
            (
                res.parse::<usize>().unwrap(),
                operands
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();
    result
}

fn solve(input: &str, operations: &[char]) -> Option<usize> {
    let result: usize = parse_input(input)
        .iter()
        .filter(|(res, operands)| guess_operator(operands, *res, operations))
        .map(|(res, _)| res)
        .sum();
    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, &['+', '*'])
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, &['+', '*', 'c'])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
