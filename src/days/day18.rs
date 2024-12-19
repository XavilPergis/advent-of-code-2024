use std::{collections::VecDeque, fmt::Display};

use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

const MAP_WIDTH: usize = 71;

fn idx(x: usize, y: usize) -> usize {
    MAP_WIDTH * y + x
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
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
    do_search(&obstacle_map, &mut weights, &mut queue);
    let answer = *weights.last().unwrap();
    Ok(answer)
}

fn do_search(obstacles: &Bitset, weights: &mut [u32], queue: &mut VecDeque<(u8, u8, u32)>) {
    weights.fill(u32::MAX);
    queue.clear();
    queue.push_back((0, 0, 0));

    while let Some((x, y, depth)) = queue.pop_front() {
        let [x, y] = [x as usize, y as usize];
        if obstacles.get(idx(x, y)) || depth >= weights[idx(x, y)] {
            continue;
        }
        weights[idx(x, y)] = depth;
        if x < MAP_WIDTH - 1 {
            queue.push_back(((x + 1) as u8, y as u8, depth + 1));
        }
        if x > 0 {
            queue.push_back(((x - 1) as u8, y as u8, depth + 1));
        }
        if y < MAP_WIDTH - 1 {
            queue.push_back((x as u8, (y + 1) as u8, depth + 1));
        }
        if y > 0 {
            queue.push_back((x as u8, (y - 1) as u8, depth + 1));
        }
    }
}

fn set_obstacles(obstacle_list: &[[u8; 2]], obstacles: &mut Bitset, limit: usize) {
    obstacles.clear_all();
    for &[x, y] in obstacle_list.iter().take(limit) {
        obstacles.set(idx(x as usize, y as usize));
    }
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut obstacles = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut weights = vec![u32::MAX; MAP_WIDTH * MAP_WIDTH];
    let mut queue = VecDeque::new();
    let mut obstacle_list = vec![];

    for line in ctx.input.lines() {
        let Some((x, y)) = line.split_once(',') else {
            eyre::bail!("invalid input");
        };
        let x = x.parse::<u8>()?;
        let y = y.parse::<u8>()?;
        obstacle_list.push([x, y]);
    }

    let mut max_reachable = 0;
    let mut min_unreachable = ctx.input.lines().count();

    while max_reachable + 1 != min_unreachable {
        obstacles.clear_all();

        let cur = (max_reachable + min_unreachable) / 2;

        set_obstacles(&obstacle_list, &mut obstacles, cur);
        do_search(&obstacles, &mut weights, &mut queue);

        if weights[weights.len() - 1] < u32::MAX {
            max_reachable = cur;
        } else {
            min_unreachable = cur;
        }
    }

    let [x, y] = obstacle_list[max_reachable];
    Ok(as_display!("{x},{y}"))
}
