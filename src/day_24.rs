use itertools::Itertools;
use std::collections::{HashMap, HashSet};
enum Operation {
    Mul,
    Eql,
    Add,
    Div,
    Mod,
}
fn perform_operation(op: Operation, a: &str, b: &str, registers: &mut [i64]) {
    let value: i64 = match b.parse::<_>() {
        Ok(number) => number,
        _ => registers[(b.chars().next().unwrap() as u8 - 'w' as u8) as usize],
    };

    let mut target: &mut i64 =
        &mut registers[(a.chars().next().unwrap() as u8 - 'w' as u8) as usize];
    match op {
        Operation::Mul => {
            *target *= value;
        }
        Operation::Eql => {
            *target = if *target == value { 1 } else { 0 };
        }
        Operation::Add => {
            *target += value;
        }
        Operation::Div => {
            *target /= value;
        }
        Operation::Mod => {
            *target %= value;
        }
    }
}

fn run_instruction(instruction: &str, registers: &mut [i64; 4]) -> usize {
    let mut params = instruction.split_whitespace();

    let operator = params.next().unwrap();
    match operator {
        "inp" => {
            //assume already filled
            //let destination = params.next().unwrap();
            //registers[(destination.chars().next().unwrap() as u8 - 'w' as u8) as usize] = input[0];
            return 1;
        }
        "mul" => {
            let a = params.next().unwrap();
            let b = params.next().unwrap();
            perform_operation(Operation::Mul, a, b, registers);
        }
        "eql" => {
            let a = params.next().unwrap();
            let b = params.next().unwrap();
            perform_operation(Operation::Eql, a, b, registers);
        }
        "add" => {
            let a = params.next().unwrap();
            let b = params.next().unwrap();
            perform_operation(Operation::Add, a, b, registers);
        }
        "div" => {
            let a = params.next().unwrap();
            let b = params.next().unwrap();
            perform_operation(Operation::Div, a, b, registers);
        }
        "mod" => {
            let a = params.next().unwrap();
            let b = params.next().unwrap();
            perform_operation(Operation::Mod, a, b, registers);
        }
        _ => panic!("Unimplemented instruction {:?}", operator),
    }
    0
}
fn run_instructions(instructions: &str, registers: &mut [i64; 4]) -> [i64; 4] {
    let mut read_inputs = 0;
    for i in instructions.lines() {
        read_inputs = run_instruction(i, registers);
    }
    *registers
}

type Cache = HashMap<(i64, usize), Option<i64>>;
fn find_modelnum(
    memo: &mut Cache,
    blocks: &[String],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if let Some(&answer) = memo.get(&(z, block)) {
        return answer;
    }
    for &digit in range {
        let mut regs = [digit, 0, 0, z];
        run_instructions(&blocks[block], &mut regs);
        let z = regs[3];
        if block + 1 == blocks.len() {
            if z == 0 {
                memo.insert((z, block), Some(digit));
                return Some(digit);
            }
            continue;
        }
        if let Some(best) = find_modelnum(memo, blocks, block + 1, z, range) {
            memo.insert((z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    memo.insert((z, block), None);
    None
}

fn model_number(instructions: &str, biggest: bool) -> String {
    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    let blocks = read_subroutines(instructions);

    let answer = find_modelnum(&mut Cache::new(), &blocks, 0, 0, &range).unwrap();
    answer.to_string().chars().rev().collect()
}

fn read_subroutines(instructions: &str) -> Vec<String> {
    let instruction_vec: Vec<String> = instructions.lines().map(|s| s.to_string()).collect();
    let mut subroutines = vec![];
    for chunk in instruction_vec.chunks(18) {
        subroutines.push(chunk.iter().fold(
            "".to_string(),
            |acc, s| if !acc.is_empty() { acc + "\n" } else { acc } + s,
        ));
    }

    subroutines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual() {
        let instructions = include_str!("../input/day_24.txt");
        //assert_eq!("99911993949684", model_number(instructions, true));
        assert_eq!("99911993949684", model_number(instructions, false));
    }
}
