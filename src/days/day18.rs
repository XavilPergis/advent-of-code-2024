use std::collections::VecDeque;

use crate::{bitset::Bitset, RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

const MAP_WIDTH: usize = 71;

fn idx(x: usize, y: usize) -> usize {
    MAP_WIDTH * y + x
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut obstacle_map = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut weights = vec![u32::MAX; MAP_WIDTH * MAP_WIDTH];
    for line in ctx.input.lines().take(1024) {
        let Some((x, y)) = line.split_once(',') else {
            eyre::bail!("invalid input");
        };
        let x = x.parse::<usize>()?;
        let y = y.parse::<usize>()?;
        // println!("({x}, {y})");
        obstacle_map.set(MAP_WIDTH * y + x);
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    while let Some((x, y, depth)) = queue.pop_front() {
        if obstacle_map.get(idx(x, y)) || depth >= weights[idx(x, y)] {
            continue;
        }
        weights[idx(x, y)] = depth;
        if x < MAP_WIDTH - 1 {
            queue.push_back((x + 1, y, depth + 1));
        }
        if x > 0 {
            queue.push_back((x - 1, y, depth + 1));
        }
        if y < MAP_WIDTH - 1 {
            queue.push_back((x, y + 1, depth + 1));
        }
        if y > 0 {
            queue.push_back((x, y - 1, depth + 1));
        }
    }

    let answer = *weights.last().unwrap();
    Ok(answer as u64)
}
