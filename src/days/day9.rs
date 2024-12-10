use std::collections::VecDeque;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut stack = VecDeque::new();
    let input = ctx.input.trim().as_bytes();
    let mut is_space = false;
    let mut id = 0;
    for ch in input {
        let num = (ch - b'0') as usize;
        if is_space && num != 0 {
            stack.push_back((0, num));
        } else {
            id += 1;
            stack.push_back((id, num));
        }
        is_space = !is_space;
    }

    let mut sum = 0;
    let mut pos = 0;
    while !stack.is_empty() {
        let (id, mut len) = stack.pop_front().unwrap();
        if id == 0 {
            // if stack.is_empty() || len == 0 {
            //     break;
            // }
            loop {
                let (mut id, mut len2) = stack.pop_back().unwrap();
                while id == 0 {
                    (id, len2) = stack.pop_back().unwrap();
                }
                if len >= len2 {
                    len -= len2;
                    sum += (id - 1) * len2 * (2 * pos + len2 - 1);
                    pos += len2;
                } else {
                    sum += (id - 1) * len * (2 * pos + len - 1);
                    pos += len;
                    stack.push_back((id, len2 - len));
                    break;
                }
            }
        } else {
            sum += (id - 1) * len * (2 * pos + len - 1);
            pos += len;
        }
    }

    // NOTE: the arithmetic sequence formula has a 0.5 factor, so we sum up twice the value for each
    // entry and divide it out here. prolly doesnt actually matter that much for perf lol.
    sum /= 2;

    Ok(sum as u64)
}
