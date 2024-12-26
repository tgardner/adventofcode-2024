use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(24);

fn parse(input: &str) -> (&str, Vec<Vec<&str>>) {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let gates = suffix
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    (prefix, gates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (prefix, gates) = parse(input);

    let mut todo: VecDeque<_> = gates.into_iter().collect();
    let mut cache = vec![u8::MAX; 1 << 15];
    let mut result = 0;

    let to_index = |s: &str| {
        let b = s.as_bytes();
        ((b[0] as usize & 31) << 10) + ((b[1] as usize & 31) << 5) + (b[2] as usize & 31)
    };

    for line in prefix.lines() {
        let prefix = &line[..3];
        let suffix = &line[5..];
        cache[to_index(prefix)] = suffix.parse().unwrap();
    }

    while let Some(gate) = todo.pop_front() {
        let [left, kind, right, _, to]: [&str; 5] = gate.as_slice().try_into().unwrap();
        let left = cache[to_index(left)];
        let right = cache[to_index(right)];

        if left == u8::MAX || right == u8::MAX {
            todo.push_back(gate);
        } else {
            cache[to_index(to)] = match kind {
                "AND" => left & right,
                "OR" => left | right,
                "XOR" => left ^ right,
                _ => unreachable!(),
            }
        }
    }

    for i in (to_index("z00")..to_index("z64")).rev() {
        if cache[i] != u8::MAX {
            result = (result << 1) | (cache[i] as u64);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = parse(input);

    let mut lookup = HashSet::new();
    let mut swapped = HashSet::new();

    for gate in &gates {
        let [left, kind, right, _, _]: [&str; 5] = gate.as_slice().try_into().unwrap();
        lookup.insert((left, kind));
        lookup.insert((right, kind));
    }

    for gate in &gates {
        let [left, kind, right, _, to]: [&str; 5] = gate.as_slice().try_into().unwrap();
        if kind == "AND" {
            // Check that all AND gates point to an OR, except for first AND.
            if left != "x00" && right != "x00" && !lookup.contains(&(to, "OR")) {
                swapped.insert(to);
            }
        }

        if kind == "OR" {
            // Check that only XOR gates point to output, except for last carry which is OR.
            if to.starts_with('z') && to != "z45" {
                swapped.insert(to);
            }
            // OR can never point to OR.
            if lookup.contains(&(to, "OR")) {
                swapped.insert(to);
            }
        }

        if kind == "XOR" {
            if left.starts_with('x') || right.starts_with('x') {
                // Check that first level XOR points to second level XOR, except for first XOR.
                if left != "x00" && right != "x00" && !lookup.contains(&(to, "XOR")) {
                    swapped.insert(to);
                }
            } else {
                // Second level XOR must point to output.
                if !to.starts_with('z') {
                    swapped.insert(to);
                }
            }
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    Some(result.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("z00,z01,z02,z03,z04".to_string()));
    }
}
