use std::collections::BTreeMap;

advent_of_code::solution!(9);

fn parse_blocks(raw_input: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();

    let mut file_id: usize = 0;
    for (idx, c) in raw_input.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            let block = if idx % 2 == 0 {
                let b = Some(file_id);
                file_id += 1;
                b
            } else {
                None
            };

            for _ in 0..digit {
                blocks.push(block);
            }
        } else {
            break;
        }
    }

    blocks
}

fn parse_files_and_free_spaces(raw_input: &str) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut file_layout = Vec::new();
    let mut free_space_layout = Vec::new();

    let mut block_id: usize = 0;
    for (idx, c) in raw_input.chars().enumerate() {
        if let Some(size) = c.to_digit(10) {
            if size > 0 {
                let size = size as usize;
                if idx % 2 == 0 {
                    file_layout.push((block_id, size));
                } else {
                    free_space_layout.push((block_id, size));
                }

                block_id += size;
            }
        } else {
            break;
        }
    }

    (file_layout, free_space_layout)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = parse_blocks(input);

    // compaction with fragmentation
    let mut block_id = 0;
    let mut checksum = 0;

    while block_id < blocks.len() {
        if let Some(file_id) = blocks[block_id] {
            checksum += file_id * block_id;
            block_id += 1;
        } else if let Some(file_id) = blocks.pop().unwrap() {
            blocks[block_id] = Some(file_id);
        }
    }
    Some(checksum as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    // files =>  block_id, size
    // free spaces => block_id, size
    let (files, free_spaces) = parse_files_and_free_spaces(input);

    // maps free space starting at block X to free space size
    let mut free_spaces_by_block_id: BTreeMap<usize, usize> = BTreeMap::from_iter(free_spaces);

    let mut checksum = 0;

    // compaction without fragmentation
    for (file_id, &(file_block_id, file_size)) in files.iter().enumerate().rev() {
        // are there any free blocks before this file that can fit it?
        let free_space = free_spaces_by_block_id
            .iter()
            .take_while(|&(&block_id, _)| block_id < file_block_id)
            .find(|&(_, &size)| size >= file_size);

        let mut new_file_block_id = file_block_id;

        // If we found a free space...
        if let Some((&free_space_block_id, &free_space_size)) = free_space {
            // 1. store the file in the free space
            new_file_block_id = free_space_block_id;

            // 2. remove the free space we've used
            free_spaces_by_block_id.remove(&free_space_block_id);

            // 3. add any remaining space behind the file back into the free spaces btree
            if free_space_size > file_size {
                free_spaces_by_block_id
                    .insert(free_space_block_id + file_size, free_space_size - file_size);
            }
        }

        // use arithmetic series formula for checksum
        checksum += (file_id as usize) * (new_file_block_id * 2 + file_size - 1) * file_size / 2;
    }
    Some(checksum)
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
