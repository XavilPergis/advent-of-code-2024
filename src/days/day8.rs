use std::collections::HashMap;

use crate::{bitset::FixedBitset, RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
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
    for (&ch, antenna_positions) in &positions {
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

    // for y in 0..50 {
    //     for x in 0..50 {
    //         print!(
    //             "{}",
    //             match antinodes.get(50 * y + x) {
    //                 true => '#',
    //                 false => ' ',
    //             }
    //         );
    //     }
    //     println!();
    // }

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
    for (&ch, antenna_positions) in &positions {
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

                let a2 = (antenna_positions[j].1 - dy, antenna_positions[j].0 - dx);
            }
        }
    }

    // for y in 0..50 {
    //     for x in 0..50 {
    //         print!(
    //             "{}",
    //             match antinodes.get(50 * y + x) {
    //                 true => '#',
    //                 false => ' ',
    //             }
    //         );
    //     }
    //     println!();
    // }

    let total = antinodes.count_ones();
    Ok(total as u64)
}
