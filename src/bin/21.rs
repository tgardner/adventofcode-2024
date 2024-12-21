advent_of_code::solution!(21);

use std::collections::{HashMap, VecDeque};

use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

fn find_shortest_paths(keypad: &Grid<u8>, from: u8, to: u8) -> Vec<Vec<u8>> {
    // find 'from' and 'to' on keypad
    let start = keypad.find(from).unwrap();
    let end = keypad.find(to).unwrap();

    if start == end {
        return vec![vec![b'A']];
    }

    // flood fill keypad to find the shortest distances
    let mut dists = keypad.same_size_with(usize::MAX);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((p, steps)) = queue.pop_front() {
        dists[p] = steps;
        for d in ORTHOGONAL {
            let np = p + d;
            if keypad.contains(np) && keypad[np] != b' ' && dists[np] == usize::MAX {
                queue.push_back((np, steps + 1));
            }
        }
    }

    // backtrack from 'end' back to 'start' and collect all paths
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((end, vec![b'A']));
    while let Some((p, path)) = stack.pop() {
        if p == start {
            paths.push(path);
            continue;
        }
        for d in ORTHOGONAL {
            let np = p + d;
            if keypad.contains(np) && dists[np] < dists[p] {
                let c = match d {
                    UP => b'v',
                    DOWN => b'^',
                    LEFT => b'>',
                    RIGHT => b'<',
                    _ => panic!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push((np, new_path));
            }
        }
    }

    paths
}

fn find_shortest_sequence(
    s: &[u8],
    depth: usize,
    highest: bool,
    cursors: &mut Vec<u8>,
    numeric: &Grid<u8>,
    directional: &Grid<u8>,
    cache: &mut HashMap<(Vec<u8>, usize, u8), usize>,
) -> usize {
    let cache_key = (s.to_vec(), depth, cursors[depth]);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    for &c in s {
        let paths = find_shortest_paths(
            if highest { numeric } else { directional },
            cursors[depth],
            c,
        );
        if depth == 0 {
            result += paths.into_iter().map(|l| l.len()).min().unwrap();
        } else {
            result += paths
                .into_iter()
                .map(|p| {
                    find_shortest_sequence(
                        &p,
                        depth - 1,
                        false,
                        cursors,
                        numeric,
                        directional,
                        cache,
                    )
                })
                .min()
                .unwrap();
        }
        cursors[depth] = c;
    }

    cache.insert(cache_key, result);

    result
}

fn solve(input: &str, max_depth: u32) -> Option<usize> {
    let codes = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();

    let numeric = Grid {
        bytes: "789456123 0A".as_bytes().to_vec(),
        width: 3,
        height: 4,
    };
    let directional = Grid {
        bytes: " ^A<v>".as_bytes().to_vec(),
        width: 3,
        height: 2,
    };
    let mut cache = HashMap::new();

    let mut total = 0;
    for code in &codes {
        let mut cursors = vec![b'A'; max_depth as usize + 1];
        let len = find_shortest_sequence(
            code.as_bytes(),
            max_depth as usize,
            true,
            &mut cursors,
            &numeric,
            &directional,
            &mut cache,
        );

        let n = code[0..3].parse::<usize>().unwrap();
        total += n * len;
    }

    Some(total)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
