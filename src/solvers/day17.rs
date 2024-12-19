use std::u64;

use itertools::Itertools;

fn combo(operand: u64, registers: &[u64; 3]) -> u64 {
    match operand {
        // literal
        0..=3 => operand,
        // registers
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        // reserved
        7 => unimplemented!("should not appear in valid programs"),
        _ => unreachable!(),
    }
}
fn execute_cycle(
    instr: u64,
    mut operand: u64,
    registers: &mut [u64; 3],
    pointer: &mut usize,
    output: &mut Vec<u64>,
) {
    match instr {
        // adv
        0 => {
            operand = combo(operand, registers);
            registers[0] = registers[0] / 2u64.pow(operand.try_into().unwrap());
            *pointer += 2;
        }
        // bxl
        1 => {
            registers[1] = registers[1] ^ operand;
            *pointer += 2;
        }
        // bst
        2 => {
            operand = combo(operand, registers) % 8;
            registers[1] = operand;
            *pointer += 2;
        }
        // jnz
        3 => {
            if registers[0] != 0 {
                *pointer = operand as usize;
            } else {
                *pointer += 2;
            }
        }
        // bxc
        4 => {
            registers[1] = registers[1] ^ registers[2];
            *pointer += 2;
        }
        // out
        5 => {
            operand = combo(operand, registers) % 8;
            output.push(operand);
            *pointer += 2;
        }
        // bdv
        6 => {
            operand = combo(operand, registers);
            registers[1] = registers[0] / 2u64.pow(operand.try_into().unwrap());
            *pointer += 2;
        }
        // cdv
        7 => {
            operand = combo(operand, registers);
            registers[2] = registers[0] / 2u64.pow(operand.try_into().unwrap());
            *pointer += 2;
        }
        _ => unreachable!(),
    }
}

fn parse_input(input: String) -> ([u64; 3], Vec<u64>) {
    let (register_string, opcode_string) = input.split_once("\n\n").unwrap();

    let mut registers = [0u64; 3];
    register_string.lines().enumerate().for_each(|(idx, line)| {
        let (_, value) = line.split_once(": ").unwrap();
        registers[idx] = value.parse().unwrap();
    });

    let actions = opcode_string
        .split_once(": ")
        .unwrap()
        .1
        .trim_end()
        .split(',')
        .map(|word| word.parse().unwrap())
        .collect::<Vec<u64>>();
    (registers, actions)
}
fn run_program(mut registers: &mut [u64; 3], opcodes: &Vec<u64>) -> Vec<u64> {
    let mut pointer = 0;
    let mut output = vec![];
    while pointer < opcodes.len() {
        let opcode = opcodes[pointer];
        let operand = opcodes[pointer + 1];
        execute_cycle(opcode, operand, &mut registers, &mut pointer, &mut output);
    }
    output
}
pub fn part1(input: String) -> String {
    let (mut registers, opcodes) = parse_input(input);

    run_program(&mut registers, &opcodes)
        .into_iter()
        .join(",")
        .to_string()
}

pub fn part2(input: String) -> String {
    let (original_registers, opcodes) = parse_input(input);

    // From messing around with the value of register A, we can observe that
    // programs seem so do *something* based on A, such that the program outputs
    // an opcode/instruction based on 3 bits (blocks).

    // While these values affect each other, i.e. there may be false positives
    // of blocks that then fail to produce the correct result with another block
    // after it. This is however limited so that we can still test combinations
    // of these blocks in a short time, which means we can simply DFS our way
    // through it, if we also keep track of up to what index each candidate
    // produces the correct output
    let mut to_test = vec![(0, 0)];
    let mut final_result = u64::MAX;
    while let Some((indexes_correct, candidate)) = to_test.pop() {
        for i in 0b000..=0b111 {
            let a_register_test = candidate << 3 | i;

            let mut registers = original_registers.clone();
            registers[0] = a_register_test;

            let output = run_program(&mut registers, &opcodes);

            if output[0] == opcodes[opcodes.len() - 1 - indexes_correct] {
                if output.len() == opcodes.len() {
                    final_result = final_result.min(a_register_test);
                    continue;
                }
                to_test.push((indexes_correct + 1, a_register_test));
            }
        }
    }
    final_result.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .to_string();
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");

        let input = "Register A: 12345678
Register B: 0
Register C: 0

Program: 2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0 "
            .to_string();
        assert_eq!(part1(input), "6,0,4,5,4,5,2,0");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .to_string();
        assert_eq!(part2(input), "117440");

        let input = "Register A: 12345678
Register B: 0
Register C: 0

Program: 2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0"
            .to_string();
        assert_eq!(part2(input), "202797954918051");
    }
}
