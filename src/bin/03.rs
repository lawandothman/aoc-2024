use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");
    let total: i32 = re
        .captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let mut enabled = true;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        if cap.get(1).is_some() {
            if enabled {
                let x: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                let y: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
                total += x * y;
            }
        } else if cap.get(4).is_some() {
            enabled = true;
        } else if cap.get(5).is_some() {
            enabled = false;
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
