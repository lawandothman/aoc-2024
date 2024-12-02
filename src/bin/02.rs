advent_of_code::solution!(2);

fn is_safe(levels: &[i32]) -> bool {
    let is_increasing = levels.windows(2).all(|pair| pair[1] > pair[0]);
    let is_decreasing = levels.windows(2).all(|pair| pair[1] < pair[0]);

    if !is_increasing && !is_decreasing {
        return false;
    }

    levels.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        (1..=3).contains(&diff)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|report| {
            let levels: Vec<i32> = report
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("Invalid number"))
                .collect();

            is_safe(&levels)
        })
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|report| {
            let levels: Vec<i32> = report
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("Invalid number"))
                .collect();

            if is_safe(&levels) {
                return true;
            }

            for i in 0..levels.len() {
                let mut modified = levels.clone();
                modified.remove(i);

                if is_safe(&modified) {
                    return true;
                }
            }
            false
        })
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
