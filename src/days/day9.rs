use std::collections::VecDeque;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

#[inline]
fn checksum_contiguous(id: usize, pos: usize, len: usize) -> usize {
    id * len * (2 * pos + len - 1) / 2
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
                    sum += checksum_contiguous(id - 1, pos, len2);
                    pos += len2;
                } else {
                    sum += checksum_contiguous(id - 1, pos, len);
                    pos += len;
                    stack.push_back((id, len2 - len));
                    break;
                }
            }
        } else {
            sum += checksum_contiguous(id - 1, pos, len);
            pos += len;
        }
    }

    // // NOTE: the arithmetic sequence formula has a 0.5 factor, so we sum up twice the value for each
    // // entry and divide it out here. prolly doesnt actually matter that much for perf lol.
    // sum /= 2;

    Ok(sum as u64)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    #[derive(Copy, Clone, Debug)]
    struct File {
        id: usize,
        pos: usize,
        size: usize,
    }
    #[derive(Copy, Clone, Debug)]
    struct Space {
        pos: usize,
        size: usize,
    }

    let mut files = vec![];
    let mut spaces = vec![];

    let input = ctx.input.trim().as_bytes();
    let mut is_space = false;
    let mut id = 0;
    let mut pos = 0;

    // NOTE: the longest a gap could be is 9, because there are no files with length of 0.

    for ch in input {
        let size = (ch - b'0') as usize;
        if is_space {
            if size != 0 {
                spaces.push(Space { pos, size });
            }
        } else {
            assert!(size != 0);
            files.push(File { id, pos, size });
            id += 1;
        }
        pos += size;
        is_space = !is_space;
    }

    for i in (0..files.len()).rev() {
        let file = &mut files[i];
        for j in 0..spaces.len() {
            let space = &mut spaces[j];
            if space.pos < file.pos && space.size >= file.size {
                file.pos = space.pos;
                space.size -= file.size;
                space.pos += file.size;
                break;
            }
        }
    }

    let checksum: usize = files
        .iter()
        .map(|file| checksum_contiguous(file.id, file.pos, file.size))
        .sum();
    // let checksum = checksum / 2;

    Ok(checksum as u64)
}
