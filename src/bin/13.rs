use itertools::Itertools;

advent_of_code::solution!(13);

fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    for l in input.split("\n\n") {
        let (x1, x2, y1, y2, z1, z2) = l
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();

        result += solve(x1, x2, y1, y2, z1, z2);
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result = 0;
    for l in input.split("\n\n") {
        let (x1, x2, y1, y2, z1, z2) = l
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();
        result += solve(x1, x2, y1, y2, z1 + 10000000000000, z2 + 10000000000000);
    }

    Some(result as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
