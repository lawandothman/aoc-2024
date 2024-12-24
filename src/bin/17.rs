advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Program {
    registers: [i64; 3],
    instructions: Vec<u8>,
}

fn parse_input(input: &str) -> Program {
    let mut registers = [0i64; 3];
    let mut instructions = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if let Some(x) = line.strip_prefix("Register A:") {
            registers[0] = x.trim().parse().expect("Invalid Register A");
        } else if let Some(x) = line.strip_prefix("Register B:") {
            registers[1] = x.trim().parse().expect("Invalid Register B");
        } else if let Some(x) = line.strip_prefix("Register C:") {
            registers[2] = x.trim().parse().expect("Invalid Register C");
        } else if let Some(x) = line.strip_prefix("Program:") {
            instructions = x
                .split(',')
                .map(|p| p.trim().parse::<u8>().expect("Invalid program byte"))
                .collect();
        }
    }

    Program {
        registers,
        instructions,
    }
}

fn run_program_for_outputs(mut prog: Program) -> Vec<i64> {
    let [ref mut a, ref mut b, ref mut c] = prog.registers;
    let mut ip = 0usize;
    let mut output = Vec::new();

    let value_of = |op: u8, a: i64, b: i64, c: i64| match op {
        0..=3 => op as i64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid combo operand {}", op),
    };

    while ip + 1 < prog.instructions.len() {
        let opcode = prog.instructions[ip];
        let operand = prog.instructions[ip + 1];

        match opcode {
            0 => {
                let exp = value_of(operand, *a, *b, *c);
                *a /= 2i64.pow(exp as u32);
            }
            1 => {
                *b ^= operand as i64;
            }
            2 => {
                *b = value_of(operand, *a, *b, *c) % 8;
            }
            3 => {
                if *a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => {
                *b ^= *c;
            }
            5 => {
                output.push(value_of(operand, *a, *b, *c) % 8);
            }
            6 => {
                let exp = value_of(operand, *a, *b, *c);
                *b = *a / 2i64.pow(exp as u32);
            }
            7 => {
                let exp = value_of(operand, *a, *b, *c);
                *c = *a / 2i64.pow(exp as u32);
            }
            _ => panic!("Invalid opcode {}", opcode),
        }

        ip += 2;
    }

    output
}

fn part_one(input: &str) -> Option<String> {
    let prog = parse_input(input);
    let outputs = run_program_for_outputs(prog.clone());
    Some(
        outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

fn part_two(_input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
