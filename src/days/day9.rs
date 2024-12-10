use std::collections::VecDeque;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part1_stackless", part1_stackless);
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

fn part1_stackless(ctx: &mut RunContext) -> eyre::Result<u64> {
    let input = ctx.input.trim().as_bytes();

    // odd length means last elem represents a file.
    assert!(input.len() & 1 == 1);

    let mut sum = 0;
    let mut pos = 0;

    let mut fi = 0;
    let mut bi = input.len() - 1;
    while fi + 1 <= bi {
        let size = (ctx.input_scratch[fi] - b'0') as usize;
        sum += checksum_contiguous(fi / 2, pos, size);
        pos += size;

        let mut space_size = (ctx.input_scratch[fi + 1] - b'0') as usize;
        loop {
            let file_size = (ctx.input_scratch[bi] - b'0') as usize;
            if space_size >= file_size {
                sum += checksum_contiguous(bi / 2, pos, file_size);
                pos += file_size;
                space_size -= file_size;
                bi -= 2;
            } else {
                sum += checksum_contiguous(bi / 2, pos, space_size);
                pos += space_size;
                ctx.input_scratch[bi] = (file_size - space_size) as u8 + b'0';
                break;
            }
        }

        fi += 2;
    }

    let size = (ctx.input_scratch[fi] - b'0') as usize;
    sum += checksum_contiguous(fi / 2, pos, size);

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

    // let mut stuck = 0;
    // let mut moved = 0;
    // let mut ksum = 0;

    // let mut ssi = 0; // space start index
    // for i in (0..files.len()).rev() {
    //     let file = &mut files[i];
    //     let mut found_min = false;
    //     let mut k = 0;
    //     let mut stuck2 = false;
    //     for j in ssi..spaces.len() {
    //         k += 1;
    //         let space = &mut spaces[j];
    //         if space.pos >= file.pos {
    //             stuck += 1;
    //             stuck2 = true;
    //             break;
    //         }
    //         if !found_min && space.size != 0 {
    //             if ssi != j {
    //                 println!();
    //                 println!("ssi = {j} delta={} size={}", j - ssi, space.size);
    //             }
    //             ssi = j;
    //             found_min = true;
    //         }
    //         if space.size >= file.size {
    //             file.pos = space.pos;
    //             space.size -= file.size;
    //             space.pos += file.size;
    //             moved += 1;
    //             break;
    //         }
    //     }
    //     ksum += k;
    //     print!("{}{k}:{} ", if stuck2 { 's' } else { 'f' }, file.size);
    // }
    // println!();

    // println!("stuck={stuck}, moved={moved}, total={}", files.len());
    // println!("kratio = {}", ksum as f32 / files.len() as f32);

    // let mut ssi = 0; // space start index
    // for i in (0..files.len()).rev() {
    //     let file = &mut files[i];
    //     let mut found_min = false;
    //     for j in ssi..spaces.len() {
    //         let space = &mut spaces[j];
    //         if space.pos >= file.pos {
    //             break;
    //         }
    //         if !found_min && space.size != 0 {
    //             ssi = j;
    //             found_min = true;
    //         }
    //         if space.size >= file.size {
    //             file.pos = space.pos;
    //             space.size -= file.size;
    //             space.pos += file.size;
    //             break;
    //         }
    //     }
    // }

    let mut space_starts = [0; 10];
    for i in (0..files.len()).rev() {
        let file = &mut files[i];

        let mut min_space_start = spaces.len();
        for i in file.size..space_starts.len() {
            min_space_start = space_starts[i].min(min_space_start);
        }

        for j in min_space_start..spaces.len() {
            let space = &mut spaces[j];
            if space.pos >= file.pos {
                break;
            }

            if space.size >= file.size {
                let new_space_size = space.size - file.size;
                // huh? why does this work?
                space_starts[space.size] = j;
                if j < space_starts[new_space_size] {
                    space_starts[new_space_size] = j;
                }
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

    Ok(checksum as u64)
}
