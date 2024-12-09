use std::collections::HashMap;

advent_of_code::solution!(8);

struct City {
    map: Vec<Vec<char>>,
    antennas: Vec<(char, usize, usize)>,
    rows: usize,
    cols: usize,
}

impl City {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let rows = map.len();
        let cols = map[0].len();
        let mut antennas = Vec::new();

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                if c.is_alphanumeric() {
                    antennas.push((*c, row_idx, col_idx));
                }
            }
        }

        Self {
            map,
            antennas,
            rows,
            cols,
        }
    }

    fn calculate_antinodes(&mut self, part2: bool) {
        let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for &(freq, x, y) in self.antennas.iter() {
            freq_map.entry(freq).or_default().push((x, y));
        }

        let range = if part2 {
            (-100..=100).collect::<Vec<_>>()
        } else {
            [-1, 2].to_vec()
        };

        for positions in freq_map.values() {
            for &(x1, y1) in positions {
                for &(x2, y2) in positions.iter().filter(|&&(x2, y2)| (x1, y1) != (x2, y2)) {
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    let positions = range
                        .iter()
                        .map(|&i| (x1 as isize + i * dx, y1 as isize + i * dy))
                        .collect::<Vec<_>>();

                    for &(nx, ny) in &positions {
                        if nx >= 0 && nx < self.rows as isize && ny >= 0 && ny < self.cols as isize
                        {
                            self.map[ny as usize][nx as usize] = '#';
                        }
                    }
                }
            }
        }
    }

    fn antinodes(&self) -> usize {
        self.map
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == '#')
            .count()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut city = City::new(input);
    city.calculate_antinodes(false);
    Some(city.antinodes() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut city = City::new(input);
    city.calculate_antinodes(true);
    Some(city.antinodes() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
