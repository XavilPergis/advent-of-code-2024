use std::simd::{cmp::SimdPartialEq, u8x32};

use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
    repo.add_variant("part2_no_recursion", part2_no_recursion);
    repo.add_variant("part2_no_recursion_flat", part2_no_recursion_flat);
    repo.add_variant("part2_no_recursion_flat_dir", part2_no_recursion_flat_dir);
    repo.add_variant(
        "part2_no_recursion_flat_unsafe",
        part2_no_recursion_flat_unsafe,
    );
}

const MAP_WIDTH: usize = 55;
// const MAP_WIDTH: usize = 8;
const MAP_STRIDE: usize = MAP_WIDTH + 1;

fn trailhead_score(map: &[u8], visited: &mut Bitset, x: usize, y: usize, cur: u8) -> usize {
    if cur == b'9' {
        let seen = visited.get(MAP_WIDTH * y + x);
        visited.set(MAP_WIDTH * y + x);
        return !seen as usize;
        // return 1;
    }

    let mut score = 0;
    if x < MAP_WIDTH - 1 && map[MAP_STRIDE * y + x + 1] == cur + 1 {
        score += trailhead_score(map, visited, x + 1, y, cur + 1);
    }
    if x > 0 && map[MAP_STRIDE * y + x - 1] == cur + 1 {
        score += trailhead_score(map, visited, x - 1, y, cur + 1);
    }
    if y < MAP_WIDTH - 1 && map[MAP_STRIDE * (y + 1) + x] == cur + 1 {
        score += trailhead_score(map, visited, x, y + 1, cur + 1);
    }
    if y > 0 && map[MAP_STRIDE * (y - 1) + x] == cur + 1 {
        score += trailhead_score(map, visited, x, y - 1, cur + 1);
    }

    score
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut visited = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut sum = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] == b'0' {
                visited.clear_all();
                sum += trailhead_score(ctx.input_scratch, &mut visited, x, y, b'0');
            }
        }
    }
    Ok(sum)
}

fn trailhead_rating(map: &[u8], x: usize, y: usize, cur: u8) -> usize {
    if cur == b'9' {
        return 1;
    }

    let mut score = 0;
    if x < MAP_WIDTH - 1 && map[MAP_STRIDE * y + x + 1] == cur + 1 {
        score += trailhead_rating(map, x + 1, y, cur + 1);
    }
    if x > 0 && map[MAP_STRIDE * y + x - 1] == cur + 1 {
        score += trailhead_rating(map, x - 1, y, cur + 1);
    }
    if y < MAP_WIDTH - 1 && map[MAP_STRIDE * (y + 1) + x] == cur + 1 {
        score += trailhead_rating(map, x, y + 1, cur + 1);
    }
    if y > 0 && map[MAP_STRIDE * (y - 1) + x] == cur + 1 {
        score += trailhead_rating(map, x, y - 1, cur + 1);
    }

    score
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut sum = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] == b'0' {
                sum += trailhead_rating(ctx.input_scratch, x, y, b'0');
            }
        }
    }
    Ok(sum)
}

fn part2_no_recursion(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut sum = 0;
    let mut stack = vec![];
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] != b'0' {
                continue;
            }
            stack.push((x, y));
            while let Some((x, y)) = stack.pop() {
                let cur = ctx.input_scratch[MAP_STRIDE * y + x];
                if cur == b'9' {
                    sum += 1;
                    continue;
                }
                if x < MAP_WIDTH - 1 && ctx.input_scratch[MAP_STRIDE * y + x + 1] == cur + 1 {
                    stack.push((x + 1, y));
                }
                if x > 0 && ctx.input_scratch[MAP_STRIDE * y + x - 1] == cur + 1 {
                    stack.push((x - 1, y));
                }
                if y < MAP_WIDTH - 1 && ctx.input_scratch[MAP_STRIDE * (y + 1) + x] == cur + 1 {
                    stack.push((x, y + 1));
                }
                if y > 0 && ctx.input_scratch[MAP_STRIDE * (y - 1) + x] == cur + 1 {
                    stack.push((x, y - 1));
                }
            }
        }
    }
    Ok(sum)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

fn part2_no_recursion_flat(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut sum = 0;
    let mut stack = Vec::with_capacity(256);

    const ZERO_CHUNK: u8x32 = u8x32::from_array([b'0'; 32]);

    let mut ix = 0;
    while ix < ctx.input_scratch.len() {
        let chunk = u8x32::load_or_default(&ctx.input_scratch[ix..]);
        if let Some(off) = chunk.simd_eq(ZERO_CHUNK).first_set() {
            ix += off;
        } else {
            ix += 32;
            continue;
        }
        debug_assert_eq!(ctx.input_scratch[ix], b'0');
        stack.push(ix as u16);
        // let mut k = 0;
        while let Some(ix) = stack.pop() {
            let ix = ix as usize;
            // k += 1;
            let cur = ctx.input_scratch[ix];
            if cur == b'9' {
                sum += 1;
                continue;
            }
            if ix + 1 < ctx.input_scratch.len() && ctx.input_scratch[ix + 1] == cur + 1 {
                stack.push((ix + 1) as u16);
            }
            if ix >= 1 && ctx.input_scratch[ix - 1] == cur + 1 {
                stack.push((ix - 1) as u16);
            }
            if ix + MAP_STRIDE < ctx.input_scratch.len()
                && ctx.input_scratch[ix + MAP_STRIDE] == cur + 1
            {
                stack.push((ix + MAP_STRIDE) as u16);
            }
            if ix >= MAP_STRIDE && ctx.input_scratch[ix - MAP_STRIDE] == cur + 1 {
                stack.push((ix - MAP_STRIDE) as u16);
            }
        }
        // print!("{k} ");
        ix += 1;
    }
    // println!();

    Ok(sum)
}

fn part2_no_recursion_flat_dir(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut sum = 0;
    let mut stack = Vec::with_capacity(256);

    const ZERO_CHUNK: u8x32 = u8x32::from_array([b'0'; 32]);

    let mut ix = 0;
    while ix < ctx.input_scratch.len() {
        let chunk = u8x32::load_or_default(&ctx.input_scratch[ix..]);
        if let Some(off) = chunk.simd_eq(ZERO_CHUNK).first_set() {
            ix += off;
        } else {
            ix += 32;
            continue;
        }
        debug_assert_eq!(ctx.input_scratch[ix], b'0');
        stack.push((ix, Direction::None));
        let mut k = 0;
        while let Some((ix, from_dir)) = stack.pop() {
            k += 1;
            let cur = ctx.input_scratch[ix];
            if cur == b'9' {
                sum += 1;
                continue;
            }

            let u = from_dir != Direction::Down && ix >= MAP_STRIDE;
            let d = from_dir != Direction::Up && ix + MAP_STRIDE < ctx.input_scratch.len();
            let l = from_dir != Direction::Right && ix >= 1;
            let r = from_dir != Direction::Left && ix + 1 < ctx.input_scratch.len();

            if r && ctx.input_scratch[ix + 1] == cur + 1 {
                stack.push((ix + 1, Direction::Right));
            }
            if l && ctx.input_scratch[ix - 1] == cur + 1 {
                stack.push((ix - 1, Direction::Left));
            }
            if d && ctx.input_scratch[ix + MAP_STRIDE] == cur + 1 {
                stack.push((ix + MAP_STRIDE, Direction::Down));
            }
            if u && ctx.input_scratch[ix - MAP_STRIDE] == cur + 1 {
                stack.push((ix - MAP_STRIDE, Direction::Up));
            }
        }
        print!("{k} ");
        ix += 1;
    }
    println!();

    Ok(sum)
}

fn part2_no_recursion_flat_unsafe(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut sum = 0;
    let mut stack = Vec::with_capacity(512);

    const ZERO_CHUNK: u8x32 = u8x32::from_array([b'0'; 32]);

    let mut ix = 0;
    while ix < ctx.input_scratch.len() {
        let chunk = u8x32::load_or_default(unsafe { ctx.input_scratch.get_unchecked(ix..) });
        if let Some(off) = chunk.simd_eq(ZERO_CHUNK).first_set() {
            ix += off;
        } else {
            ix += 32;
            continue;
        }
        debug_assert_eq!(ctx.input_scratch[ix], b'0');
        stack.push(ix);
        while let Some(ix) = stack.pop() {
            let cur = unsafe { *ctx.input_scratch.get_unchecked(ix) };
            if cur == b'9' {
                sum += 1;
                continue;
            }
            if ix + 1 < ctx.input_scratch.len()
                && unsafe { *ctx.input_scratch.get_unchecked(ix + 1) } == cur + 1
            {
                stack.push(ix + 1);
            }
            if ix >= 1 && unsafe { *ctx.input_scratch.get_unchecked(ix - 1) } == cur + 1 {
                stack.push(ix - 1);
            }
            if ix + MAP_STRIDE < ctx.input_scratch.len()
                && unsafe { *ctx.input_scratch.get_unchecked(ix + MAP_STRIDE) } == cur + 1
            {
                stack.push(ix + MAP_STRIDE);
            }
            if ix >= MAP_STRIDE
                && unsafe { *ctx.input_scratch.get_unchecked(ix - MAP_STRIDE) } == cur + 1
            {
                stack.push(ix - MAP_STRIDE);
            }
        }
        ix += 1;
    }

    Ok(sum)
}
