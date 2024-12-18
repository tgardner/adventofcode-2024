use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

advent_of_code::solution!(12);

pub fn parse(input: &str) -> (usize, usize) {
    let grid = Grid::parse(input);

    let mut todo = Vec::new();
    let mut edge = Vec::new();
    let mut seen = grid.same_size_with(false);

    let mut part_one = 0;
    let mut part_two = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already filled points.
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            // Flood fill, using area as an index.
            let kind = grid[point];
            let check = |point| grid.contains(point) && grid[point] == kind;

            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            todo.push(point);
            seen[point] = true;

            while area < todo.len() {
                let point = todo[area];
                area += 1;

                for direction in ORTHOGONAL {
                    let next = point + direction;

                    if check(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        edge.push((point, direction));
                        perimeter += 1;
                    }
                }
            }

            // Sum sides for all plots in the region.
            for &(p, d) in &edge {
                let r = d.clockwise();
                let l = d.counter_clockwise();

                sides += (!check(p + l) || check(p + l + d)) as usize;
                sides += (!check(p + r) || check(p + r + d)) as usize;
            }

            todo.clear();
            edge.clear();

            part_one += area * perimeter;
            part_two += area * (sides / 2);
        }
    }

    (part_one, part_two)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (part_one, _) = parse(input);
    Some(part_one as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, part_two) = parse(input);
    Some(part_two as u32)
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
