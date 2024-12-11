use std::collections::HashMap;

advent_of_code::solution!(11);

fn solve(input: &str, blinks: u8) -> Option<usize> {
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for stone in input.split_whitespace() {
        let number = stone.parse().unwrap();
        *stones.entry(number).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut next_stones = HashMap::new();

        for (&stone, &count) in &stones {
            if stone == 0 {
                *next_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let left = digits[..mid].parse().unwrap();
                let right = digits[mid..].parse().unwrap();
                *next_stones.entry(left).or_insert(0) += count;
                *next_stones.entry(right).or_insert(0) += count;
            } else {
                *next_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stones = next_stones;
    }

    Some(stones.values().sum())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
