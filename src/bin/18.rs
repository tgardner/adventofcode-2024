use std::collections::VecDeque;

use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

advent_of_code::solution!(18);

const ORIGIN: Point = Point::new(0, 0);

fn parse(input: &str, grid_size: i32) -> Grid<u16> {
    let mut grid = Grid::new(grid_size, grid_size, u16::MAX);
    input.lines().enumerate().for_each(|(id, line)| {
        let p: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        grid[Point::new(p[0], p[1])] = (id + 1) as u16;
    });
    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input, 71);
    bfs(&grid, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    let grid = parse(input, 71);
    find_choke_point(&grid)
}

fn bfs(grid: &Grid<u16>, time: u16) -> Option<u32> {
    let mut todo = VecDeque::new();
    let mut seen = grid.clone();

    todo.push_back((ORIGIN, 0));
    seen[ORIGIN] = 0;

    while let Some((p, cost)) = todo.pop_front() {
        if (p.x, p.y) == (grid.width - 1, grid.height - 1) {
            return Some(cost);
        }

        for next in ORTHOGONAL.map(|o| p + o) {
            if grid.contains(next) && time < seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = 0;
            }
        }
    }

    None
}

fn find_choke_point(grid: &Grid<u16>) -> Option<String> {
    let mut lower: u16 = 0;
    let mut upper: u16 = (grid.width * grid.height) as u16;

    while lower < upper {
        let middle = (lower + upper) / 2;
        if bfs(&grid, middle).is_some() {
            lower = middle + 1;
        } else {
            upper = middle;
        }
    }

    if let Some(p) = grid.find(lower) {
        return Some(format!("{},{}", p.x, p.y));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(input, 7);
        let result = bfs(&grid, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(input, 7);
        let result = find_choke_point(&grid);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
