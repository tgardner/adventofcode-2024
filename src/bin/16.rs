use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

use advent_of_code::Direction;

advent_of_code::solution!(16);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    cost: usize,
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();
        let start = find_char(&grid, 'S');
        let end = find_char(&grid, 'E');
        let direction = Direction::Right;
        Maze {
            grid,
            start,
            end,
            direction,
        }
    }
}

impl Maze {
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn solve(&self) -> usize {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let (sx, sy) = self.start;
        let (ex, ey) = self.end;
    
        // Priority queue: Min-heap of (cost + heuristic, cost, state)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0 + heuristic(sx, sy, ex, ey), 0, State { x: sx, y: sy, direction: self.direction, cost: 0 })));
    
        let mut visited = HashSet::new();
    
        while let Some(Reverse((_, cost, state))) = heap.pop() {
            if visited.contains(&(state.x, state.y, state.direction)) {
                continue;
            }
            visited.insert((state.x, state.y, state.direction));
    
            // Goal check
            if (state.x, state.y) == (ex, ey) {
                return cost;
            }
    
            // Move forward
            let (nx, ny) = state.direction.apply(state.x as isize, state.y as isize);
    
            if nx >= 0 && ny >= 0 && (nx as usize) < cols && (ny as usize) < rows && self.grid[ny as usize][nx as usize] != '#' {
                heap.push(Reverse((
                    cost + 1 + heuristic(nx as usize, ny as usize, ex, ey),
                    cost + 1,
                    State { x: nx as usize, y: ny as usize, direction: state.direction, cost: cost + 1 },
                )));
            }
    
            // Turn left or right
            for turn in [-1, 1] {
                let new_direction = if turn > 0 {
                    state.direction.rot90()
                } else {
                    state.direction.rot270()
                };
                heap.push(Reverse((
                    cost + 1000 + heuristic(state.x, state.y, ex, ey),
                    cost + 1000,
                    State { x: state.x, y: state.y, direction: new_direction, cost: cost + 1000 },
                )));
            }
        }
    
        usize::MAX // If no path found
    }
}

fn heuristic(x: usize, y: usize, ex: usize, ey: usize) -> usize {
    (x.abs_diff(ex) + y.abs_diff(ey)) as usize
}

fn find_char(grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == c {
                return (x, y);
            }
        }
    }
    (0, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::from(input);
    let result = maze.solve();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
