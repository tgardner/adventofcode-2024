advent_of_code::solution!(4);

fn count_xmas_occurrences(grid: &[Vec<char>], directions: &[(i32, i32)], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in directions {
                if (0..word_len).all(|i| {
                    let new_row = row as isize + dr as isize * i as isize;
                    let new_col = col as isize + dc as isize * i as isize;
                    new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                        && grid[new_row as usize][new_col as usize] == word_chars[i]
                }) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..(rows - 2) {
        for col in 0..(cols - 2) {
            if is_xmas_pattern(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_xmas_pattern(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let top_left = grid[row][col];
    let top_right = grid[row][col + 2];
    let middle = grid[row + 1][col + 1];
    let bottom_left = grid[row + 2][col];
    let bottom_right = grid[row + 2][col + 2];

    middle == 'A'
        && ((top_left == 'M' && bottom_right == 'S' && top_right == 'M' && bottom_left == 'S')
            || (top_left == 'S' && bottom_right == 'M' && top_right == 'S' && bottom_left == 'M')
            || (top_left == 'M' && bottom_right == 'S' && top_right == 'S' && bottom_left == 'M')
            || (top_left == 'S' && bottom_right == 'M' && top_right == 'M' && bottom_left == 'S'))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
    ];

    let count = count_xmas_occurrences(&grid, &directions, "XMAS");
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let count = count_xmas_patterns(&grid);
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
