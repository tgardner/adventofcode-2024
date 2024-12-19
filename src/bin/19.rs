advent_of_code::solution!(19);

fn parse(input: &str) -> Vec<usize> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let towels: Vec<_> = prefix.split(", ").collect();
    suffix
        .lines()
        .map(|design| count_ways_dp(design, &towels))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let possible_designs = parse(input).iter().filter(|c| **c > 0).count();
    Some(possible_designs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let total_ways = parse(input).iter().sum();
    Some(total_ways)
}

fn count_ways_dp(design: &str, patterns: &[&str]) -> usize {
    let n = design.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1; // 1 way to make an empty design

    for i in 0..n {
        if dp[i] > 0 {
            for pattern in patterns {
                if i + pattern.len() <= n && design[i..].starts_with(pattern) {
                    dp[i + pattern.len()] += dp[i];
                }
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
