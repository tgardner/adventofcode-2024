use std::collections::VecDeque;

use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

advent_of_code::solution!(16);

/// Clockwise order starting with facing right.
const DIRECTIONS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

pub fn parse(input: &str) -> (u32, usize) {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Forwards Dijkstra. Since turns are so much more expensive than moving forward, we can
    // treat this as a glorified BFS using two priority queues. This is much faster than using
    // an actual min heap.
    let mut todo_first = VecDeque::new();
    let mut todo_second = VecDeque::new();
    // State is `(position, direction)`.
    let mut seen = grid.same_size_with([u32::MAX; 4]);
    let mut lowest = u32::MAX;

    todo_first.push_back((start, 0, 0));
    seen[start][0] = 0;

    while !todo_first.is_empty() {
        while let Some((position, direction, cost)) = todo_first.pop_front() {
            if cost >= lowest {
                continue;
            }
            if position == end {
                lowest = cost;
                continue;
            }

            // -1.rem_euclid(4) = 3
            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for tuple @ (next_position, next_direction, next_cost) in next {
                if grid[next_position] != b'#' && next_cost < seen[next_position][next_direction] {
                    // Find the next bucket.
                    if next_direction == direction {
                        todo_first.push_back(tuple);
                    } else {
                        todo_second.push_back(tuple);
                    }
                    seen[next_position][next_direction] = next_cost;
                }
            }
        }

        (todo_first, todo_second) = (todo_second, todo_first);
    }

    // Backwards BFS
    let mut todo = VecDeque::new();
    let mut path = grid.same_size_with(false);

    // Lowest paths can arrive at end node in multiple directions.
    for direction in 0..4 {
        if seen[end][direction] == lowest {
            todo.push_back((end, direction, lowest));
        }
    }

    while let Some((position, direction, cost)) = todo.pop_front() {
        path[position] = true;
        if position == start {
            continue;
        }

        // Reverse direction and subtract cost.
        let left = (direction + 3) % 4;
        let right = (direction + 1) % 4;
        let next = [
            (position - DIRECTIONS[direction], direction, cost - 1),
            (position, left, cost - 1000),
            (position, right, cost - 1000),
        ];

        for (next_position, next_direction, next_cost) in next {
            // Trace our cost step by step so it will exactly match possible paths.
            if next_cost == seen[next_position][next_direction] {
                todo.push_back((next_position, next_direction, next_cost));
                // Set cost back to `u32::MAX` to prevent redundant path explorations.
                seen[next_position][next_direction] = u32::MAX;
            }
        }
    }

    (lowest, path.bytes.iter().filter(|&&b| b).count())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (lowest, _) = parse(input);
    Some(lowest)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, path_len) = parse(input);
    Some(path_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
