use advent_of_code::DIRECTIONS;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_price(grid: &[Vec<char>], part2: bool) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                // BFS to find the region
                let plant_type = grid[r][c];
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                visited[r][c] = true;

                let mut area = 0;
                let mut perimeter = 0;
                let mut region = HashSet::new();

                while let Some((x, y)) = queue.pop_front() {
                    region.insert((x as i32, y as i32));
                    area += 1;
                    for &(dx, dy) in &DIRECTIONS {
                        let nx = x as isize + dx as isize;
                        let ny = y as isize + dy as isize;

                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if grid[nx][ny] == plant_type && !visited[nx][ny] {
                                visited[nx][ny] = true;
                                queue.push_back((nx, ny));
                            } else if grid[nx][ny] != plant_type {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                if part2 {
                    total_price += area * count_sides(&region);
                } else {
                    total_price += area * perimeter;
                }
            }
        }
    }

    total_price
}

fn count_sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut edge_count = 0;
    for &(r, c) in region {
        let up = r > 0 && region.contains(&(r - 1, c));
        let down = region.contains(&(r + 1, c));
        let left = c > 0 && region.contains(&(r, c - 1));
        let right = region.contains(&(r, c + 1));
        let up_left = r > 0 && c > 0 && region.contains(&(r - 1, c - 1));
        let up_right = r > 0 && region.contains(&(r - 1, c + 1));
        let down_left = c > 0 && region.contains(&(r + 1, c - 1));
        let down_right = region.contains(&(r + 1, c + 1));

        if !up && !right || up && right && !up_right {
            edge_count += 1;
        }
        if !up && !left || up && left && !up_left {
            edge_count += 1;
        }
        if !down && !right || down && right && !down_right {
            edge_count += 1;
        }
        if !down && !left || down && left && !down_left {
            edge_count += 1;
        }
    }
    edge_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_map(input);
    let total_price = calculate_price(&grid, false);
    Some(total_price as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_map(input);
    let total_price = calculate_price(&grid, true);
    Some(total_price as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
