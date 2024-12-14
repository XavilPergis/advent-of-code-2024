use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

fn parse_number(text: &[u8], cur: &mut usize) -> i32 {
    let mut res = 0;
    let mut mul = 1;
    if *cur < text.len() && text[*cur] == b'-' {
        *cur += 1;
        mul = -1;
    }
    while *cur < text.len() && text[*cur].is_ascii_digit() {
        res *= 10;
        res += (text[*cur] - b'0') as i32;
        *cur += 1;
    }
    mul * res
}

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for line in ctx.input.lines() {
        let line = line.as_bytes();
        let mut cur = 0;
        cur += 2; // "p="
        let x = parse_number(line, &mut cur);
        cur += 1; // ","
        let y = parse_number(line, &mut cur);
        cur += 3; // " v="
        let vx = parse_number(line, &mut cur);
        cur += 1; // ","
        let vy = parse_number(line, &mut cur);

        let final_x = (x + 100 * vx).rem_euclid(MAP_WIDTH);
        let final_y = (y + 100 * vy).rem_euclid(MAP_HEIGHT);

        if final_x < (MAP_WIDTH - 1) / 2 && final_y < (MAP_HEIGHT - 1) / 2 {
            q1 += 1;
        }
        if final_x > (MAP_WIDTH - 1) / 2 && final_y < (MAP_HEIGHT - 1) / 2 {
            q2 += 1;
        }
        if final_x < (MAP_WIDTH - 1) / 2 && final_y > (MAP_HEIGHT - 1) / 2 {
            q3 += 1;
        }
        if final_x > (MAP_WIDTH - 1) / 2 && final_y > (MAP_HEIGHT - 1) / 2 {
            q4 += 1;
        }
    }
    
    Ok((q1 * q2 * q3 * q4) as u64)
}
