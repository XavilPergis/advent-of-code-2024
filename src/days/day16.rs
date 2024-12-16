use crate::{bitset::Bitset, RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

fn pathfind_dfs(
    walls: &Bitset,
    cost_map: &mut [usize],
    pos: usize,
    dir: usize,
    goal: usize,
    cost: usize,
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

    let cw = dir.wrapping_add(1) & 3;
    let ccw = dir.wrapping_sub(1) & 3;

    pathfind_dfs(
        walls,
        cost_map,
        pos.wrapping_add_signed(OFFSETS[dir]),
        dir,
        goal,
        cost + 1,
    );
    // add 1001 to cost instead of the 1000 incurred by turning, because we *also* do a move into the adjacent tile.
    pathfind_dfs(
        walls,
        cost_map,
        pos.wrapping_add_signed(OFFSETS[cw]),
        cw,
        goal,
        cost + 1001,
    );
    pathfind_dfs(
        walls,
        cost_map,
        pos.wrapping_add_signed(OFFSETS[ccw]),
        ccw,
        goal,
        cost + 1001,
    );
}

const MAP_WIDTH: usize = 141;
const OFFSETS: [isize; 4] = [1, MAP_WIDTH as isize, -1, -(MAP_WIDTH as isize)];

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut cost_map = vec![usize::MAX; MAP_WIDTH * MAP_WIDTH];
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
    Ok(score as u64)
}
