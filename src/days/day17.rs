use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
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
            let num = state.registers[REG_A];
            let den = 1 << combo_op(&state, operand);
            state.registers[REG_A] = num / den;
        }
        OP_BDV => {
            let num = state.registers[REG_A];
            let den = 1 << combo_op(&state, operand);
            state.registers[REG_B] = num / den;
        }
        OP_CDV => {
            let num = state.registers[REG_A];
            let den = 1 << combo_op(&state, operand);
            state.registers[REG_C] = num / den;
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

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut state = State {
        ip: 0,
        registers: [0, 0, 0],
        program: vec![],
        output: vec![],
    };

    let mut cur = 0;
    cur += 12; // "Register A: "
    state.registers[REG_A] = parse_number(&ctx.input_scratch, &mut cur);
    cur += 13; // "\nRegister B: "
    state.registers[REG_B] = parse_number(&ctx.input_scratch, &mut cur);
    cur += 13; // "\nRegister C: "
    state.registers[REG_C] = parse_number(&ctx.input_scratch, &mut cur);
    cur += 11; // "\n\nProgram: "

    while cur < ctx.input_scratch.len() {
        state.program.push(ctx.input_scratch[cur + 0] - b'0');
        state.program.push(ctx.input_scratch[cur + 2] - b'0');
        cur += 4;
    }

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