advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let input: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut file_id: usize = 0;

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10)? as usize;
        if i % 2 == 0 {
            for _ in 0..count {
                blocks.push(Some(file_id));
            }
            file_id = file_id.wrapping_add(1);
        } else {
            for _ in 0..count {
                blocks.push(None);
            }
        }
    }

    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].is_none() {
            if let Some(j) = blocks.iter().rposition(|b| b.is_some()) {
                if j <= i {
                    break;
                }
                blocks[i] = blocks[j];
                blocks[j] = None;
            } else {
                break;
            }
        }
        i += 1;
    }

    let checksum: u128 = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|id| (id as u128).saturating_mul(pos as u128)))
        .sum();

    usize::try_from(checksum).ok()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut file_id: usize = 0;
    let mut file_ranges = Vec::new();

    let mut current_pos = 0;
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10)? as usize;
        if i % 2 == 0 {
            if count > 0 {
                let start = current_pos;
                for _ in 0..count {
                    blocks.push(Some(file_id));
                    current_pos += 1;
                }
                file_ranges.push((file_id, start, count));
            } else {
                file_ranges.push((file_id, current_pos, 0));
            }
            file_id = file_id.wrapping_add(1);
        } else {
            for _ in 0..count {
                blocks.push(None);
                current_pos += 1;
            }
        }
    }

    if let Some(max_id) = file_ranges.iter().map(|f| f.0).max() {
        for id in (0..=max_id).rev() {
            let (_, start, length) = file_ranges[id];
            if length == 0 {
                continue;
            }
            if let Some(free_start) = find_free_span(&blocks, length, start) {
                for offset in 0..length {
                    blocks[free_start + offset] = blocks[start + offset];
                    blocks[start + offset] = None;
                }
                file_ranges[id] = (id, free_start, length);
            }
        }
    }

    let checksum: u128 = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &b)| b.map(|id| (id as u128) * (pos as u128)))
        .sum();

    usize::try_from(checksum).ok()
}

fn find_free_span(blocks: &[Option<usize>], length: usize, end_limit: usize) -> Option<usize> {
    let mut consecutive = 0;

    for (i, _block) in blocks.iter().enumerate().take(end_limit) {
        if blocks[i].is_none() {
            consecutive += 1;
            if consecutive == length {
                return Some(i - length + 1);
            }
        } else {
            consecutive = 0;
        }
    }

    None
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
