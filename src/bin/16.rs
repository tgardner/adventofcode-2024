use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::Direction;

advent_of_code::solution!(16);

struct Maze {
    grid: Vec<Vec<char>>,
    free_spaces: HashSet<(isize, isize)>,
    start: (isize, isize),
    end: (isize, isize),
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut free_spaces = HashSet::new();
        let mut start = (-1, -1);
        let mut end = (-1, -1);

        for (y, l) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (x, c) in l.chars().enumerate() {
                grid[y].push(c);
                match c {
                    'E' => {
                        end = (x as isize, y as isize);
                        free_spaces.insert((x as isize, y as isize));
                    }
                    'S' => {
                        start = (x as isize, y as isize);
                    }
                    '.' => {
                        free_spaces.insert((x as isize, y as isize));
                    }
                    _ => {}
                }
            }
        }

        Maze {
            grid,
            start,
            end,
            free_spaces,
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

    fn dijkstra(&self) -> HashMap<((isize, isize), Direction), isize> {
        let mut to_visit = BinaryHeap::new();
        let mut visited: HashMap<((isize, isize), Direction), isize> = HashMap::new();
        visited.insert((self.start, Direction::Right), 0);

        to_visit.push((0, Direction::Right, self.start));

        while let Some((score, cd, (cx, cy))) = to_visit.pop() {
            let score = -score; // Negate score due to BinaryHeap being a min-heap

            if visited.get(&((cx, cy), cd)).map_or(false, |&v| v < score) {
                continue;
            }

            // Try forward
            let np = cd.apply(cx, cy);
            if self.free_spaces.contains(&np)
                && visited.get(&(np, cd)).map_or(true, |&v| v > score + 1)
            {
                visited.insert((np, cd), score + 1);
                to_visit.push((-(score + 1), cd, np));
            }

            // Try turn
            for nd in [cd.rot90(), cd.rot270()] {
                if visited
                    .get(&((cx, cy), nd))
                    .map_or(true, |&v| v > score + 1000)
                {
                    visited.insert(((cx, cy), nd), score + 1000);
                    to_visit.push((-(score + 1000), nd, (cx, cy)));
                }
            }
        }

        visited
    }
}

fn trace_back(
    visited: &HashMap<((isize, isize), Direction), isize>,
    target_state: ((isize, isize), Direction),
) -> HashSet<(isize, isize)> {
    let mut to_visit = vec![target_state];
    let mut seen = HashSet::new();

    while let Some((cp, cd)) = to_visit.pop() {
        seen.insert(cp);

        let np = cd.rot180().apply(cp.0, cp.1);

        // Try back forward
        if visited
            .get(&(np, cd))
            .map_or(false, |&v| v + 1 == visited[&(cp, cd)])
        {
            to_visit.push((np, cd));
        }

        // Try rotate
        for nd in [cd.rot90(), cd.rot270()] {
            if visited
                .get(&(cp, nd))
                .map_or(false, |&v| v + 1000 == visited[&(cp, cd)])
            {
                to_visit.push((cp, nd));
            }
        }
    }

    seen
}

pub fn part_one(input: &str) -> Option<isize> {
    let maze = Maze::from(input);
    let visited = maze.dijkstra();
    let min_score = visited
        .iter()
        .filter(|&(&(pos, _), _)| pos == maze.end)
        .map(|(_, &score)| score)
        .min();
    min_score
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::from(input);
    let visited = maze.dijkstra();
    let target_score = visited
        .iter()
        .filter(|&(&(pos, _), _)| pos == maze.end)
        .map(|(_, &score)| score)
        .min()
        .unwrap();

    let target_state = visited
        .iter()
        .find(|&(&(pos, _), &score)| pos == maze.end && score == target_score)
        .map(|(&(pos, dir), _)| (pos, dir))
        .unwrap();

    let path_len = trace_back(&visited, target_state).len();
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
