use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn pathfind_dfs(
    walls: &Bitset,
    cost_map: &mut [u32],
    pos: usize,
    dir: usize,
    goal: usize,
    cost: u32,
) {
    if walls.get(pos) {
        return;
    }

    // dfs search, if we find a node that has a lower cost than the current cost, abort searching further.
    // NOTE: this is not optimal!

    // abort if we couldn't possibly find a better path by recursing deeper.
    if cost > cost_map[pos] {
        return;
    } else {
        cost_map[pos] = cost;
    }

    // we don't need to do any more searching if we reached the goal; any more steps would be unoptimal.
    if pos == goal {
        return;
    }

    let cw_dir = dir.wrapping_add(1) & 3;
    let ccw_dir = dir.wrapping_sub(1) & 3;

    let front_pos = pos.wrapping_add_signed(OFFSETS[dir]);
    let right_pos = pos.wrapping_add_signed(OFFSETS[cw_dir]);
    let left_pos = pos.wrapping_add_signed(OFFSETS[ccw_dir]);

    pathfind_dfs(walls, cost_map, front_pos, dir, goal, cost + 1);
    // add 1001 to cost instead of the 1000 incurred by turning, because we *also* do a move into the adjacent tile.
    pathfind_dfs(walls, cost_map, right_pos, cw_dir, goal, cost + 1001);
    pathfind_dfs(walls, cost_map, left_pos, ccw_dir, goal, cost + 1001);
}

const MAP_WIDTH: usize = 141;
const OFFSETS: [isize; 4] = [1, MAP_WIDTH as isize, -1, -(MAP_WIDTH as isize)];

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut cost_map = vec![u32::MAX; MAP_WIDTH * MAP_WIDTH];
    let mut map = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut start_pos = 0;
    let mut goal_pos = 0;

    let mut ix = 0;
    for i in 0..ctx.input_scratch.len() {
        match ctx.input_scratch[i] {
            b'\n' => continue,
            b'#' => map.set(ix),
            b'S' => start_pos = ix,
            b'E' => goal_pos = ix,
            _ => {}
        }
        ix += 1;
    }

    pathfind_dfs(&map, &mut cost_map, start_pos, 0, goal_pos, 0);
    let score = cost_map[goal_pos];
    Ok(score)
}

fn pathfind_dfs_directional(
    walls: &Bitset,
    cost_map: &mut [[u32; 4]],
    pos: usize,
    dir: usize,
    goal: usize,
    cost: u32,
) {
    if walls.get(pos) {
        return;
    }

    // dfs search, if we find a node that has a lower cost than the current cost, abort searching further.
    // NOTE: this is not optimal!

    // abort if we couldn't possibly find a better path by recursing deeper.
    let cw_dir = dir.wrapping_add(1) & 3;
    let ccw_dir = dir.wrapping_sub(1) & 3;
    let back_dir = dir.wrapping_add(2) & 3;
    let low_cost = cost.saturating_sub(1001);
    if cost > cost_map[pos][dir]
        || low_cost > cost_map[pos][back_dir]
        || low_cost > cost_map[pos][cw_dir]
        || low_cost > cost_map[pos][ccw_dir]
    {
        return;
    }
    cost_map[pos][dir] = cost;

    // we don't need to do any more searching if we reached the goal; any more steps would be unoptimal.
    if pos == goal {
        return;
    }

    let front_pos = pos.wrapping_add_signed(OFFSETS[dir]);
    let cw_pos = pos.wrapping_add_signed(OFFSETS[cw_dir]);
    let ccw_pos = pos.wrapping_add_signed(OFFSETS[ccw_dir]);
    pathfind_dfs_directional(walls, cost_map, front_pos, dir, goal, cost + 1);
    if !walls.get(cw_pos) {
        pathfind_dfs_directional(walls, cost_map, pos, cw_dir, goal, cost + 1000);
    }
    if !walls.get(ccw_pos) {
        pathfind_dfs_directional(walls, cost_map, pos, ccw_dir, goal, cost + 1000);
    }
}

fn mark_best_path(
    cost_map: &[[u32; 4]],
    best_path: &mut Bitset,
    pos: usize,
    dir: usize,
    goal: usize,
) {
    best_path.set(pos);
    if pos == goal {
        return;
    }

    let cur = cost_map[pos][dir];

    let back_dir = (dir + 2) & 3;
    let back_pos = pos.wrapping_add_signed(OFFSETS[back_dir]);
    if cost_map[back_pos][dir] + 1 == cur {
        mark_best_path(cost_map, best_path, back_pos, dir, goal);
    }

    let cw_dir = dir.wrapping_add(1) & 3;
    if cost_map[pos][cw_dir] + 1000 == cur {
        mark_best_path(cost_map, best_path, pos, cw_dir, goal);
    }
    let ccw_dir = dir.wrapping_sub(1) & 3;
    if cost_map[pos][ccw_dir] + 1000 == cur {
        mark_best_path(cost_map, best_path, pos, ccw_dir, goal);
    }
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut cost_map = vec![[u32::MAX; 4]; MAP_WIDTH * MAP_WIDTH];
    let mut map = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut best_path = Bitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut start_pos = 0;
    let mut goal_pos = 0;

    let mut ix = 0;
    for i in 0..ctx.input_scratch.len() {
        match ctx.input_scratch[i] {
            b'\n' => continue,
            b'#' => map.set(ix),
            b'S' => start_pos = ix,
            b'E' => goal_pos = ix,
            _ => {}
        }
        ix += 1;
    }

    pathfind_dfs_directional(&map, &mut cost_map, start_pos, 0, goal_pos, 0);
    let goal_min = cost_map[goal_pos].iter().copied().min().unwrap();
    for i in 0..4 {
        if cost_map[goal_pos][i] == goal_min {
            mark_best_path(&cost_map, &mut best_path, goal_pos, i, start_pos);
        }
    }

    let score = best_path.count_ones();
    Ok(score)
}
