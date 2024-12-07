use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let eqs = input.lines().filter_map(|l| {
        let mut parts = l.split(": ");
        let target = parts.next()?.parse::<usize>().ok()?;
        let numbers = parts
            .next()?
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<_>>();
        Some((target, numbers))
    });

    let mut result = 0;
    for (target, numbers) in eqs {
        if can_produce_target(target, &numbers, &["+", "*"]) {
            result += target;
        }
    }

    Some(result)
}

fn can_produce_target(target: usize, numbers: &[usize], operators: &[&str]) -> bool {
    let operator_combo = (0..(numbers.len() - 1))
        .map(|_| operators.iter())
        .multi_cartesian_product();

    for op in operator_combo {
        if evaluate_expression(numbers, &op) == Some(target) {
            return true;
        }
    }
    false
}

fn evaluate_expression(numbers: &[usize], operators: &[&&str]) -> Option<usize> {
    let mut result = numbers[0];

    for (&num, &op) in numbers.iter().skip(1).zip(operators.iter()) {
        result = match *op {
            "+" => result.checked_add(num)?,
            "*" => result.checked_mul(num)?,
            "||" => concatenate_numbers(result, num)?,
            _ => unreachable!(),
        };
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let eqs = input.lines().filter_map(|l| {
        let mut parts = l.split(": ");
        let target = parts.next()?.parse::<usize>().ok()?;
        let numbers = parts
            .next()?
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<_>>();
        Some((target, numbers))
    });

    let mut result = 0;
    for (target, numbers) in eqs {
        if can_produce_target(target, &numbers, &["+", "*", "||"]) {
            result += target;
        }
    }

    Some(result)
}

fn concatenate_numbers(a: usize, b: usize) -> Option<usize> {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<usize>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
