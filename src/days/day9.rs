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
        if is_space {
            if num != 0 {
                stack.push_back((0, num));
            }
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
            if stack.is_empty() {
                break;
            }
            loop {
                if len == 0 {
                    break;
                }
                let (mut id2, mut len2) = stack.pop_back().unwrap();
                while id2 == 0 {
                    (id2, len2) = stack.pop_back().unwrap();
                }
                if len >= len2 {
                    len -= len2;
                    for i in pos..pos + len2 {
                        sum += (id2 - 1) * i;
                    }
                    // sum += (id2 - 1) * (len2 * pos + len2 * len2);
                    pos += len2;
                } else {
                    // sum += (id2 - 1) * (len * pos + len * len);
                    for i in pos..pos + len {
                        sum += (id2 - 1) * i;
                    }
                    pos += len;
                    stack.push_back((id2, len2 - len));
                    len = 0;
                }
            }
        } else {
            for i in pos..pos + len {
                sum += (id - 1) * i;
            }
            // sum += (id - 1) * (len * pos + len * len);
            pos += len;
        }
    }

    Ok(sum as u64)
}
