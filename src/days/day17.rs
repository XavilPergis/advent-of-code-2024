use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn parse_number(input: &[u8], cur: &mut usize) -> u64 {
    let mut i = *cur;
    let mut res = 0;
    while input[i].is_ascii_digit() {
        res *= 10;
        res += (input[i] - b'0') as u64;
        i += 1
    }
    *cur = i;
    res
}

const OP_ADV: u8 = 0;
const OP_BXL: u8 = 1;
const OP_BST: u8 = 2;
const OP_JNZ: u8 = 3;
const OP_BXC: u8 = 4;
const OP_OUT: u8 = 5;
const OP_BDV: u8 = 6;
const OP_CDV: u8 = 7;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

struct State {
    ip: usize,
    registers: [u64; 3],
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ip: 0,
            registers: [0; 3],
            program: vec![],
            output: vec![],
        }
    }
}

// The value of a combo operand can be found as follows:
// - Combo operands 0 through 3 represent literal values 0 through 3.
// - Combo operand 4 represents the value of register A.
// - Combo operand 5 represents the value of register B.
// - Combo operand 6 represents the value of register C.
// - Combo operand 7 is reserved and will not appear in valid programs.
fn combo_op(state: &State, value: u8) -> u64 {
    match value {
        n @ 0..=3 => n as u64,
        4 => state.registers[REG_A],
        5 => state.registers[REG_B],
        6 => state.registers[REG_C],
        n => panic!("invalid combo operand: {n}"),
    }
}

fn step_program(state: &mut State) {
    let opcode = state.program[state.ip];
    let operand = state.program[state.ip + 1];

    match opcode {
        // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
        // The denominator is found by raising 2 to the power of the instruction's combo operand.
        // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
        // The result of the division operation is truncated to an integer and then written to the A register.
        OP_ADV => {
            state.registers[REG_A] = state.registers[REG_A] >> combo_op(&state, operand);
        }
        OP_BDV => {
            state.registers[REG_B] = state.registers[REG_A] >> combo_op(&state, operand);
        }
        OP_CDV => {
            state.registers[REG_C] = state.registers[REG_A] >> combo_op(&state, operand);
        }
        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B
        // and the instruction's literal operand, then stores the result in register B.
        OP_BXL => {
            state.registers[REG_B] ^= operand as u64;
        }
        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
        // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
        OP_BST => {
            state.registers[REG_B] = combo_op(&state, operand) & 7;
        }
        // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero,
        // it jumps by setting the instruction pointer to the value of its literal operand;
        // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
        OP_JNZ => {
            if state.registers[REG_A] != 0 {
                state.ip = operand as usize;
                return;
            }
        }
        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
        // then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
        OP_BXC => {
            state.registers[REG_B] ^= state.registers[REG_C];
        }
        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
        // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
        OP_OUT => {
            let value = combo_op(&state, operand) & 7;
            state.output.push(value as u8);
        }
        op => panic!("unknown opcode: {op}"),
    }

    state.ip += 2;
}

fn parse_state(input: &[u8], state: &mut State) {
    state.program.clear();
    state.output.clear();

    let mut cur = 0;
    cur += 12; // "Register A: "
    state.registers[REG_A] = parse_number(input, &mut cur);
    cur += 13; // "\nRegister B: "
    state.registers[REG_B] = parse_number(input, &mut cur);
    cur += 13; // "\nRegister C: "
    state.registers[REG_C] = parse_number(input, &mut cur);
    cur += 11; // "\n\nProgram: "

    while cur < input.len() {
        state.program.push(input[cur + 0] - b'0');
        state.program.push(input[cur + 2] - b'0');
        cur += 4;
    }
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut state = State::default();
    parse_state(&ctx.input_scratch, &mut state);

    while state.ip < state.program.len() {
        step_program(&mut state);
    }

    let mut display = String::new();
    for i in 0..state.output.len() {
        if i != 0 {
            display.push(',');
        }
        display.push((state.output[i] + b'0') as char);
    }

    println!("{display}");

    Ok(0)
}

fn do_cycle(program: &[u8], a: u64) -> u8 {
    let mut reg = [a, 0, 0];

    fn combo_op(reg: &[u64; 3], value: u8) -> u64 {
        match value {
            n @ 0..=3 => n as u64,
            n @ 4..=6 => reg[n as usize - 4],
            n => panic!("invalid combo operand: {n}"),
        }
    }

    for i in 0..program.len() / 2 {
        let opcode = program[i * 2];
        let operand = program[i * 2 + 1];
        match opcode {
            OP_ADV => reg[REG_A] = reg[REG_A] >> combo_op(&reg, operand),
            OP_BDV => reg[REG_B] = reg[REG_A] >> combo_op(&reg, operand),
            OP_CDV => reg[REG_C] = reg[REG_A] >> combo_op(&reg, operand),
            OP_BXL => reg[REG_B] ^= operand as u64,
            OP_BST => reg[REG_B] = combo_op(&reg, operand) & 7,
            OP_BXC => reg[REG_B] ^= reg[REG_C],
            OP_OUT | OP_JNZ => {}
            op => panic!("unknown opcode: {op}"),
        }
    }

    (reg[REG_B] & 7) as u8
}

// the last 7 bits of A are the only bits that contribute to the output for that particular cycle.
// out[0] = op(A0)
// A1 = A0 >> 3
// out[1] = op(A1)
// A2 = A1 >> 3
// ...
// A_n = A_(n-1) >> 3
// out_n = op(A_n)
fn test_candidate(program: &[u8], n: usize, cur_a: u64) -> Option<u64> {
    for i in 0..(1 << 3) {
        let prev_a = cur_a << 3 | i;
        // let (_, value) = do_cycle(prev_a);
        let value = do_cycle(program, prev_a);
        if program[n] == value {
            if n == 0 {
                return Some(prev_a);
            }
            if let Some(value) = test_candidate(program, n - 1, prev_a) {
                return Some(value);
            }
        }
    }
    // failed, backtrack.
    None
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut state = State::default();
    parse_state(&ctx.input_scratch, &mut state);

    let target = state.program.clone();
    let res = test_candidate(&state.program, target.len() - 1, 0).unwrap();

    // state.registers[REG_A] = res;
    // while state.ip < state.program.len() {
    //     step_program(&mut state);
    // }
    // assert_eq!(&target, &state.output);

    Ok(res)
}
