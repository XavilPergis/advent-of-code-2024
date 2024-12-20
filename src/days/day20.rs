use std::collections::VecDeque;

use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

const MAP_SIZE: usize = 141;
const MAP_STRIDE: usize = MAP_SIZE + 1;

fn idx(x: usize, y: usize) -> usize {
    MAP_SIZE * y + x
}

fn count_shortcuts(walls: &Bitset, goal_distance: &[u32], x: usize, y: usize) -> usize {
    let mut res = 0;

    if x + 2 < MAP_SIZE && walls.get(idx(x + 1, y)) && !walls.get(idx(x + 2, y)) {
        let before = goal_distance[idx(x, y)];
        let after = goal_distance[idx(x + 2, y)];
        if before > after + 2 {
            let diff = (before - 2) - after;
            if diff >= 100 {
                res += 1;
            }
        }
    }
    if x >= 2 && walls.get(idx(x - 1, y)) && !walls.get(idx(x - 2, y)) {
        let before = goal_distance[idx(x, y)];
        let after = goal_distance[idx(x - 2, y)];
        if before > after + 2 {
            let diff = (before - 2) - after;
            if diff >= 100 {
                res += 1;
            }
        }
        
    }
    if y + 2 < MAP_SIZE && walls.get(idx(x, y + 1)) && !walls.get(idx(x, y + 2)) {
        let before = goal_distance[idx(x, y)];
        let after = goal_distance[idx(x, y + 2)];
        if before > after + 2 {
            let diff = (before - 2) - after;
            if diff >= 100 {
                res += 1;
            }
        }
    }
    if y >= 2 && walls.get(idx(x, y - 1)) && !walls.get(idx(x, y - 2)) {
        let before = goal_distance[idx(x, y)];
        let after = goal_distance[idx(x, y - 2)];
        if before > after + 2 {
            let diff = (before - 2) - after;
            if diff >= 100 {
                res += 1;
            }
        }
    }

    res
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let (mut start, mut end) = (0, 0);
    let mut walls = Bitset::new(MAP_SIZE * MAP_SIZE);
    let mut goal_distance = vec![u32::MAX; MAP_SIZE * MAP_SIZE];

    let mut ix = 0;
    for i in 0..ctx.input_scratch.len() {
        match ctx.input_scratch[i] {
            b'\n' => continue,
            b'#' => walls.set(ix),
            b'S' => start = ix,
            b'E' => end = ix,
            _ => {}
        }
        ix += 1;
    }

    let mut queue = VecDeque::new();
    queue.push_back((end % MAP_SIZE, end / MAP_SIZE, 0));
    while let Some((x, y, d)) = queue.pop_front() {
        if goal_distance[idx(x, y)] < d {
            continue;
        }
        goal_distance[idx(x, y)] = d;
        if !walls.get(idx(x + 1, y)) {
            queue.push_back((x + 1, y, d + 1));
        }
        if !walls.get(idx(x - 1, y)) {
            queue.push_back((x - 1, y, d + 1));
        }
        if !walls.get(idx(x, y + 1)) {
            queue.push_back((x, y + 1, d + 1));
        }
        if !walls.get(idx(x, y - 1)) {
            queue.push_back((x, y - 1, d + 1));
        }
    }

    let mut res = 0;
    let mut queue = VecDeque::new();
    let mut visited = Bitset::new(MAP_SIZE * MAP_SIZE);
    queue.push_back((start % MAP_SIZE, start / MAP_SIZE));
    while let Some((x, y)) = queue.pop_front() {
        if visited.get(idx(x, y)) {
            continue;
        }
        visited.set(idx(x, y));
        res += count_shortcuts(&walls, &goal_distance, x, y);
        if x < MAP_SIZE - 1 && !walls.get(idx(x + 1, y)) {
            queue.push_back((x + 1, y));
        }
        if x > 0 && !walls.get(idx(x - 1, y)) {
            queue.push_back((x - 1, y));
        }
        if y < MAP_SIZE - 1 && !walls.get(idx(x, y + 1)) {
            queue.push_back((x, y + 1));
        }
        if y > 0 && !walls.get(idx(x, y - 1)) {
            queue.push_back((x, y - 1));
        }
    }

    Ok(res)
}
