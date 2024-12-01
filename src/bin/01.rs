use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l, mut r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let mut nums = l.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    l.sort();
    r.sort();

    let sum: i32 = l.iter().zip(r.iter()).map(|(l, r)| (l - r).abs()).sum();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l, r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let mut nums = l.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    let mut r_count: HashMap<i32, usize> = HashMap::new();

    for n in r.iter() {
        *r_count.entry(*n).or_insert(0) += 1;
    }

    let score: i32 = l
        .iter()
        .map(|&n| {
            let count = *r_count.get(&n).unwrap_or(&0);
            n * count as i32
        })
        .sum();

    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
