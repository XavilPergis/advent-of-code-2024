use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn memchr(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().copied().position(|ch| ch == needle)
}

const MAP_WIDTH: usize = 50;
const MAP_STRIDE: usize = MAP_WIDTH + 1;

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut i = 0;
    while i + 1 < ctx.input_scratch.len() {
        if ctx.input_scratch[i] == b'\n' && ctx.input_scratch[i + 1] == b'\n' {
            break;
        }
        i += 1;
    }

    let (map, insns) = ctx.input_scratch.split_at_mut(i + 1);

    let mut ix = memchr(map, b'@').unwrap();
    for &mut insn in insns {
        let offset = match insn {
            b'^' => -(MAP_STRIDE as isize),
            b'>' => 1,
            b'v' => MAP_STRIDE as isize,
            b'<' => -1,
            _ => continue,
        };

        let neighbor_ix = ix.wrapping_add_signed(offset);

        if map[neighbor_ix] == b'.' {
            map[ix] = b'.';
            map[neighbor_ix] = b'@';
            // free spot in front, move directly into it.
            ix = neighbor_ix;
        } else if map[neighbor_ix] == b'O' {
            // box in front, scan for next free spot and move box into that free spot,
            // stepping forward once.
            let mut scan_ix = neighbor_ix;
            while map[scan_ix] == b'O' {
                scan_ix = scan_ix.wrapping_add_signed(offset);
            }
            // if we find a free space after the line of boxes, move a box into it, and move
            // the robot into the box's previous location.
            if map[scan_ix] == b'.' {
                // found free space, move into it.
                map[ix] = b'.';
                map[neighbor_ix] = b'@';
                map[scan_ix] = b'O';
                ix = neighbor_ix;
            }
        }
    }

    let mut res = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if map[MAP_STRIDE * y + x] == b'O' {
                res += 100 * y + x;
            }
        }
    }

    Ok(res as u64)
}

// 600 boxes
// 0000 0000 empty
// 0000 0001 wall
// 0000 0010 robot
// 0000 0100 box left
// 0000 0101 box right

const TILE_EMPTY: u8 = 0x00;
const TILE_WALL: u8 = 0x01;
const TILE_ROBOT: u8 = 0x02;
const TILE_BOX_LEFT: u8 = 0x04;
const TILE_BOX_RIGHT: u8 = 0x05;

fn can_move_into(map: &[u8], pos: usize, dir: isize) -> bool {
    let target_pos = pos.wrapping_add_signed(dir);
    match map[target_pos] {
        TILE_WALL => false,
        TILE_BOX_LEFT => {
            if map[pos] == TILE_BOX_LEFT || dir == -1 {
                can_move_into(map, target_pos, dir)
            } else {
                can_move_into(map, target_pos, dir) && can_move_into(map, target_pos + 1, dir)
            }
        }
        TILE_BOX_RIGHT => {
            if map[pos] == TILE_BOX_RIGHT || dir == 1 {
                can_move_into(map, target_pos, dir)
            } else {
                can_move_into(map, target_pos, dir) && can_move_into(map, target_pos - 1, dir)
            }
        }
        _ => true,
    }
}

// assumes that can_move_into was called with the same args and returned true.
fn push_block(map: &mut [u8], pos: usize, dir: isize) {
    let target_pos = pos.wrapping_add_signed(dir);
    match map[target_pos] {
        TILE_BOX_LEFT => {
            push_block(map, target_pos, dir);
            if dir != 1 && dir != -1 {
                push_block(map, target_pos + 1, dir);
            }
        }
        TILE_BOX_RIGHT => {
            push_block(map, target_pos, dir);
            if dir != 1 && dir != -1 {
                push_block(map, target_pos - 1, dir);
            }
        }
        _ => {}
    }
    map[target_pos] = map[pos];
    map[pos] = TILE_EMPTY;
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut map = vec![];

    let mut ix = 0;
    let mut i = 0;
    while i + 1 < ctx.input_scratch.len() {
        match ctx.input_scratch[i] {
            b'.' => map.extend([TILE_EMPTY; 2]),
            b'#' => map.extend([TILE_WALL; 2]),
            b'O' => map.extend([TILE_BOX_LEFT, TILE_BOX_RIGHT]),
            b'@' => {
                ix = map.len();
                map.extend([TILE_ROBOT, TILE_EMPTY]);
            }
            _ => {}
        }
        if ctx.input_scratch[i] == b'\n' && ctx.input_scratch[i + 1] == b'\n' {
            break;
        }
        i += 1;
    }

    let (_, insns) = ctx.input_scratch.split_at(i + 1);

    for &insn in insns {
        // apply insn
        let offset = match insn {
            b'^' => -(2 * MAP_WIDTH as isize),
            b'>' => 1,
            b'v' => 2 * MAP_WIDTH as isize,
            b'<' => -1,
            _ => continue,
        };

        let neighbor_ix = ix.wrapping_add_signed(offset);

        if map[neighbor_ix] == TILE_EMPTY {
            map[ix] = TILE_EMPTY;
            map[neighbor_ix] = TILE_ROBOT;
            ix = neighbor_ix;
        } else if map[neighbor_ix] == TILE_BOX_LEFT || map[neighbor_ix] == TILE_BOX_RIGHT {
            if can_move_into(&map, ix, offset) {
                push_block(&mut map, ix, offset);
                ix = neighbor_ix;
            }
        }
    }

    let mut res = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..2 * MAP_WIDTH {
            let is_box = map[2 * MAP_WIDTH * y + x] == TILE_BOX_LEFT;
            res += is_box as usize * (100 * y + x);
        }
    }

    Ok(res as u64)
}
