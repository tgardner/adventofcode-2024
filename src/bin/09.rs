advent_of_code::solution!(9);

fn solve(mut files: Vec<(usize, i32)>) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id == -1 && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, -1);
            if size < s {
                files.insert(j + 1, (s - size, -1));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut fs1 = Vec::new();
    let mut fid = 0;

    for (i, b) in input.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            let v = if i % 2 == 0 {
                fid += 1;
                fid - 1
            } else {
                -1
            };
            fs1.extend((0..b - b'0').map(|_| (1, v)));
        }
    }

    let p1 = solve(fs1);
    Some(p1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut fs2 = Vec::new();
    let mut fid = 0;

    for (i, b) in input.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            let v = if i % 2 == 0 {
                fid += 1;
                fid - 1
            } else {
                -1
            };
            fs2.push(((b - b'0') as usize, v));
        }
    }

    let p2 = solve(fs2);
    Some(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
