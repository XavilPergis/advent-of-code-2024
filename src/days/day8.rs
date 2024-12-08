use std::collections::HashMap;

use crate::{bitset::FixedBitset, RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
    repo.add_variant("part2_no_map", part2_no_map);
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut positions = HashMap::<u8, Vec<(i32, i32)>>::new();
    let mut x = 0;
    let mut y = 0;
    for &ch in ctx.input.as_bytes() {
        if ch.is_ascii_alphanumeric() {
            positions.entry(ch).or_default().push((x, y));
        }
        x += 1;
        if ch == b'\n' {
            x = 0;
            y += 1;
        }
    }

    let mut antinodes = FixedBitset::new(50 * 50);
    for (_, antenna_positions) in &positions {
        for i in 0..antenna_positions.len() {
            for j in i + 1..antenna_positions.len() {
                let dx = antenna_positions[i].0 - antenna_positions[j].0;
                let dy = antenna_positions[i].1 - antenna_positions[j].1;
                let a1 = (antenna_positions[i].1 + dy, antenna_positions[i].0 + dx);
                let a2 = (antenna_positions[j].1 - dy, antenna_positions[j].0 - dx);
                if a1.0 >= 0 && a1.0 < 50 && a1.1 >= 0 && a1.1 < 50 {
                    antinodes.set(50 * a1.1 as usize + a1.0 as usize);
                }
                if a2.0 >= 0 && a2.0 < 50 && a2.1 >= 0 && a2.1 < 50 {
                    antinodes.set(50 * a2.1 as usize + a2.0 as usize);
                }
            }
        }
    }

    let total = antinodes.count_ones();
    Ok(total as u64)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut positions = HashMap::<u8, Vec<(i32, i32)>>::new();
    let mut x = 0;
    let mut y = 0;
    for &ch in ctx.input.as_bytes() {
        if ch.is_ascii_alphanumeric() {
            positions.entry(ch).or_default().push((x, y));
        }
        x += 1;
        if ch == b'\n' {
            x = 0;
            y += 1;
        }
    }

    let mut antinodes = FixedBitset::new(50 * 50);
    for (_, antenna_positions) in &positions {
        for i in 0..antenna_positions.len() {
            for j in i + 1..antenna_positions.len() {
                let dx = antenna_positions[i].0 - antenna_positions[j].0;
                let dy = antenna_positions[i].1 - antenna_positions[j].1;

                let (mut xr, mut yr) = (antenna_positions[i].0, antenna_positions[i].1);
                while xr >= 0 && xr < 50 && yr >= 0 && yr < 50 {
                    antinodes.set(50 * yr as usize + xr as usize);
                    xr += dx;
                    yr += dy;
                }

                let (mut xr, mut yr) = (antenna_positions[j].0, antenna_positions[j].1);
                while xr >= 0 && xr < 50 && yr >= 0 && yr < 50 {
                    antinodes.set(50 * yr as usize + xr as usize);
                    xr -= dx;
                    yr -= dy;
                }
            }
        }
    }

    let total = antinodes.count_ones();
    Ok(total as u64)
}

#[derive(Copy, Clone, Debug)]
struct FixedVec<T, const N: usize> {
    len: usize,
    data: [T; N],
}

fn part2_no_map(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut chars = FixedVec {
        len: 0,
        data: [0u8; 62],
    };
    let mut positions = [FixedVec {
        len: 0,
        data: [(0i32, 0i32); 8],
    }; 62];

    let mut x = 0;
    let mut y = 0;
    for &ch in ctx.input.as_bytes() {
        if ch.is_ascii_alphanumeric() {
            // positions.entry(ch).or_default().push((x, y));
            let mut res = None::<usize>;
            for i in 0..62 {
                if chars.data[i] == ch {
                    res = Some(i);
                }
            }
            if res.is_none() {
                res = Some(chars.len);
                chars.data[chars.len] = ch;
                chars.len += 1;
            }
            let i = res.unwrap();
            positions[i].data[positions[i].len] = (x, y);
            positions[i].len += 1;
        }
        x += 1;
        if ch == b'\n' {
            x = 0;
            y += 1;
        }
    }

    let mut antinodes = FixedBitset::new(50 * 50);
    for k in 0..chars.len {
        let antenna_positions = &positions[k].data[..positions[k].len];
        for i in 0..antenna_positions.len() {
            for j in i + 1..antenna_positions.len() {
                let dx = antenna_positions[i].0 - antenna_positions[j].0;
                let dy = antenna_positions[i].1 - antenna_positions[j].1;

                // assert!(dx != 0 || dy != 0);

                let (mut xr, mut yr) = (antenna_positions[i].0, antenna_positions[i].1);
                while xr >= 0 && xr < 50 && yr >= 0 && yr < 50 {
                    antinodes.set(50 * yr as usize + xr as usize);
                    xr += dx;
                    yr += dy;
                }

                let (mut xr, mut yr) = (antenna_positions[j].0, antenna_positions[j].1);
                while xr >= 0 && xr < 50 && yr >= 0 && yr < 50 {
                    antinodes.set(50 * yr as usize + xr as usize);
                    xr -= dx;
                    yr -= dy;
                }
            }
        }
    }

    let total = antinodes.count_ones();
    Ok(total as u64)
}
