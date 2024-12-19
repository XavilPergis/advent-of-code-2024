use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

const MAP_SIZE: usize = 140;
// const MAP_SIZE: usize = 10;
const MAP_STRIDE: usize = MAP_SIZE + 1;

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut sum = 0;
    let mut visited = Bitset::new(MAP_STRIDE * MAP_STRIDE);
    let mut stack = vec![];
    for ix in 0..ctx.input_scratch.len() {
        let cur = ctx.input_scratch[ix];
        if cur == b'\n' || visited.get(ix) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;

        stack.clear();
        stack.push(ix);
        while let Some(ix) = stack.pop() {
            if visited.get(ix) {
                continue;
            }
            visited.set(ix);
            area += 1;

            if ix + 1 < ctx.input_scratch.len() {
                if ctx.input_scratch[ix + 1] != cur {
                    perim += 1;
                } else if !visited.get(ix + 1) {
                    stack.push(ix + 1);
                }
            } else {
                perim += 1;
            }

            if ix >= 1 {
                if ctx.input_scratch[ix - 1] != cur {
                    perim += 1;
                } else if !visited.get(ix - 1) {
                    stack.push(ix - 1);
                }
            } else {
                perim += 1;
            }

            if ix + MAP_STRIDE < ctx.input_scratch.len() {
                if ctx.input_scratch[ix + MAP_STRIDE] != cur {
                    perim += 1;
                } else if !visited.get(ix + MAP_STRIDE) {
                    stack.push(ix + MAP_STRIDE);
                }
            } else {
                perim += 1;
            }

            if ix >= MAP_STRIDE {
                if ctx.input_scratch[ix - MAP_STRIDE] != cur {
                    perim += 1;
                } else if !visited.get(ix - MAP_STRIDE) {
                    stack.push(ix - MAP_STRIDE);
                }
            } else {
                perim += 1;
            }
        }

        sum += area * perim;
    }

    Ok(sum as u64)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Winding {
    Outer,
    Inner,
}

fn scan_edge(
    ctx: &mut RunContext,
    visited: &mut [Bitset; 4],
    ix: usize,
    cur: u8,
    winding: Winding,
) -> usize {
    let deltas = [1, MAP_STRIDE as isize, -1, -(MAP_STRIDE as isize)];

    let start_ix = ix;
    let mut ix = ix;
    let mut dir = 0usize;

    let mut perim = 0;

    let (wdp, wdn) = match winding {
        Winding::Outer => (-1, 1),
        Winding::Inner => (1, -1),
    };

    loop {
        // move cursor (take a single movement action before looping again)
        let side_dir = dir.wrapping_add_signed(wdp) & 3;
        let front_ix = ix as isize + deltas[dir];
        let front_side_ix = front_ix + deltas[side_dir];

        visited[side_dir].set(ix);

        'step: {
            // if its possible to move into the side of the tile in front of us, then do it and turn to the side.
            if front_side_ix >= 0 && front_side_ix < ctx.input_scratch.len() as isize {
                let is_front_wall = front_ix >= 0
                    && front_ix < ctx.input_scratch.len() as isize
                    && ctx.input_scratch[front_ix as usize] != cur;
                let front_side_ix = front_side_ix as usize;
                if !is_front_wall && ctx.input_scratch[front_side_ix] == cur {
                    ix = front_side_ix;
                    dir = dir.wrapping_add_signed(wdp) & 3;
                    perim += 1;
                    break 'step;
                }
            }
            if front_ix >= 0 && front_ix < ctx.input_scratch.len() as isize {
                let front_ix = front_ix as usize;
                // front_ix may refer to \n, which will always be different than `cur` so we don't
                // need to handle wrapping explicitly.
                if ctx.input_scratch[front_ix] != cur {
                    dir = dir.wrapping_add_signed(wdn) & 3;
                    perim += 1;
                } else {
                    ix = front_ix;
                }
                break 'step;
            }
            // if we hit a border, then we could have only done it going in the direction
            // of the border, meaning that it will always be correct to turn clockwise here.
            dir = dir.wrapping_add_signed(wdn) & 3;
            perim += 1;
        }

        // back to starting configuration, we completed the loop.
        if ix == start_ix && dir == 0 {
            break perim;
        }
    }
}

// const RIGHT: usize = 0;
const DOWN: usize = 1;
// const LEFT: usize = 2;
const UP: usize = 3;

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut counted_perims = [
        Bitset::new(MAP_STRIDE * MAP_STRIDE), // r
        Bitset::new(MAP_STRIDE * MAP_STRIDE), // d
        Bitset::new(MAP_STRIDE * MAP_STRIDE), // l
        Bitset::new(MAP_STRIDE * MAP_STRIDE), // u
    ];
    let mut sum = 0;
    let mut visited = Bitset::new(MAP_STRIDE * MAP_STRIDE);
    let mut stack = vec![];
    for ix in 0..ctx.input_scratch.len() {
        let cur = ctx.input_scratch[ix];
        if cur == b'\n' || visited.get(ix) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;

        stack.clear();
        stack.push(ix);
        while let Some(ix) = stack.pop() {
            if visited.get(ix) {
                continue;
            }
            visited.set(ix);
            area += 1;

            if ix + 1 < ctx.input_scratch.len()
                && ctx.input_scratch[ix + 1] == cur
                && !visited.get(ix + 1)
            {
                stack.push(ix + 1);
            }
            if ix >= 1 && ctx.input_scratch[ix - 1] == cur && !visited.get(ix - 1) {
                stack.push(ix - 1);
            }
            if ix + MAP_STRIDE < ctx.input_scratch.len()
                && ctx.input_scratch[ix + MAP_STRIDE] == cur
                && !visited.get(ix + MAP_STRIDE)
            {
                stack.push(ix + MAP_STRIDE);
            }
            if ix >= MAP_STRIDE
                && ctx.input_scratch[ix - MAP_STRIDE] == cur
                && !visited.get(ix - MAP_STRIDE)
            {
                stack.push(ix - MAP_STRIDE);
            }

            // we don't update the visited map for interior tiles, but we know that we've already counted
            // the region containing the interior tile, so we can just skip it.
            let is_inset_bounds = ix >= MAP_STRIDE
                && ix + MAP_STRIDE < ctx.input_scratch.len()
                && ctx.input_scratch[ix + 1] != b'\n'
                && ctx.input_scratch[ix - 1] != b'\n';
            if is_inset_bounds {
                let is_enclosed = ctx.input_scratch[ix + 1] == cur
                    && ctx.input_scratch[ix - 1] == cur
                    && ctx.input_scratch[ix + MAP_STRIDE] == cur
                    && ctx.input_scratch[ix - MAP_STRIDE] == cur;
                if is_enclosed {
                    continue;
                }
            }

            // if no edge above where there should be edge, scan outer
            if !counted_perims[UP].get(ix) {
                if ix < MAP_STRIDE || ctx.input_scratch[ix - MAP_STRIDE] != cur {
                    perim += scan_edge(ctx, &mut counted_perims, ix, cur, Winding::Outer);
                }
            }
            if !counted_perims[DOWN].get(ix) {
                if ix + MAP_STRIDE >= ctx.input_scratch.len()
                    || ctx.input_scratch[ix + MAP_STRIDE] != cur
                {
                    perim += scan_edge(ctx, &mut counted_perims, ix, cur, Winding::Inner);
                }
            }
        }

        sum += area * perim;
    }

    Ok(sum as u64)
}
