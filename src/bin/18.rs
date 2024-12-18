use std::collections::VecDeque;

use advent_of_code::Direction;

advent_of_code::solution!(18);

type Point = (usize, usize);
const ORIGIN: Point = (0, 0);

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut x = line.split(",");
            (
                x.next().unwrap().parse().unwrap(),
                x.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn part_one_solution(input: &str, grid_size: usize, bytes: usize) -> Option<u32> {
    let mut grid = vec![vec![b'.'; grid_size]; grid_size];
    let mut seen = vec![vec![usize::MAX; grid_size]; grid_size];

    parse(input).iter().take(bytes).for_each(|&p| grid[p.1][p.0] = b'#');
    bfs(&grid, &mut seen, 0)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_solution(input, 71, 1024)
}

fn parse_two_solution(input: &str, grid_size: usize) -> Option<String> {
    let mut grid = vec![vec![b'.'; grid_size]; grid_size];
    let mut seen = vec![vec![usize::MAX; grid_size]; grid_size];

    let input = parse(input);
    for (id, &(px, py)) in input.iter().enumerate() {
        grid[py][px] = b'#';
        if bfs(&grid, &mut seen, id).is_none() {
            return Some(format!("{},{}", px, py));
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    parse_two_solution(input, 71)
}

fn bfs(grid: &[Vec<u8>], seen: &mut [Vec<usize>], id: usize) -> Option<u32> {
    let mut todo = VecDeque::new();
    let (width, height) = (grid[0].len(), grid.len());
    todo.push_back((ORIGIN, 0));
    seen[ORIGIN.1][ORIGIN.0] = id;

    while let Some((position, cost)) = todo.pop_front() {
        if position == (width - 1, height - 1) {
            return Some(cost);
        }

        for (nx, ny) in
            Direction::iterator().map(|o| o.apply(position.0 as isize, position.1 as isize))
        {
            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny][nx] != b'#' && seen[ny][nx] != id {
                todo.push_back(((nx, ny), cost + 1));
                seen[ny][nx] = id;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one_solution(input, 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse_two_solution(input, 7);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
