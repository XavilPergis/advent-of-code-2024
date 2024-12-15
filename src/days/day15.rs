use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
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
