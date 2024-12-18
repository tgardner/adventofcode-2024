use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let res = solve(&grid, false);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let res = solve(&grid, true);
    Some(res)
}

fn solve(grid: &Grid<u8>, distinct: bool) -> u32 {
    let mut result = 0;
    let mut seen = grid.same_size_with(-1);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'9' {
                let id = y * grid.width + x;
                result += dfs(grid, distinct, &mut seen, id, point);
            }
        }
    }

    result
}

fn dfs(grid: &Grid<u8>, distinct: bool, seen: &mut Grid<i32>, id: i32, point: Point) -> u32 {
    let mut result = 0;

    for next in ORTHOGONAL.map(|o| point + o) {
        if grid.contains(next) && grid[next] + 1 == grid[point] && (distinct || seen[next] != id) {
            seen[next] = id;

            if grid[next] == b'0' {
                result += 1;
            } else {
                result += dfs(grid, distinct, seen, id, next);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
