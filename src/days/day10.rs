use crate::{bitset::FixedBitset, RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
    repo.add_variant("part2_no_recursion", part2_no_recursion);
}

const MAP_WIDTH: usize = 55;
// const MAP_WIDTH: usize = 8;
const MAP_STRIDE: usize = MAP_WIDTH + 1;

fn trailhead_score(map: &[u8], visited: &mut FixedBitset, x: usize, y: usize, cur: u8) -> usize {
    if cur == b'9' {
        let seen = visited.get(MAP_WIDTH * y + x);
        visited.set(MAP_WIDTH * y + x);
        return !seen as usize;
        // return 1;
    }

    let mut score = 0;
    if x < MAP_WIDTH - 1 && map[MAP_STRIDE * y + x + 1] == cur + 1 {
        score += trailhead_score(map, visited, x + 1, y, cur + 1);
    }
    if x > 0 && map[MAP_STRIDE * y + x - 1] == cur + 1 {
        score += trailhead_score(map, visited, x - 1, y, cur + 1);
    }
    if y < MAP_WIDTH - 1 && map[MAP_STRIDE * (y + 1) + x] == cur + 1 {
        score += trailhead_score(map, visited, x, y + 1, cur + 1);
    }
    if y > 0 && map[MAP_STRIDE * (y - 1) + x] == cur + 1 {
        score += trailhead_score(map, visited, x, y - 1, cur + 1);
    }

    score
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut visited = FixedBitset::new(MAP_WIDTH * MAP_WIDTH);
    let mut sum = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] == b'0' {
                visited.clear_all();
                sum += trailhead_score(ctx.input_scratch, &mut visited, x, y, b'0');
            }
        }
    }
    Ok(sum as u64)
}

fn trailhead_rating(map: &[u8], x: usize, y: usize, cur: u8) -> usize {
    if cur == b'9' {
        return 1;
    }

    let mut score = 0;
    if x < MAP_WIDTH - 1 && map[MAP_STRIDE * y + x + 1] == cur + 1 {
        score += trailhead_rating(map, x + 1, y, cur + 1);
    }
    if x > 0 && map[MAP_STRIDE * y + x - 1] == cur + 1 {
        score += trailhead_rating(map, x - 1, y, cur + 1);
    }
    if y < MAP_WIDTH - 1 && map[MAP_STRIDE * (y + 1) + x] == cur + 1 {
        score += trailhead_rating(map, x, y + 1, cur + 1);
    }
    if y > 0 && map[MAP_STRIDE * (y - 1) + x] == cur + 1 {
        score += trailhead_rating(map, x, y - 1, cur + 1);
    }

    score
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut sum = 0;
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] == b'0' {
                sum += trailhead_rating(ctx.input_scratch, x, y, b'0');
            }
        }
    }
    Ok(sum as u64)
}

fn part2_no_recursion(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut sum = 0;
    let mut stack = vec![];
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            if ctx.input_scratch[MAP_STRIDE * y + x] == b'0' {
                continue;
            }
            stack.push((x, y));
            while let Some((x, y)) = stack.pop() {
                let cur = ctx.input_scratch[MAP_STRIDE * y + x];
                if cur == b'9' {
                    sum += 1;
                    continue;
                }
                if x < MAP_WIDTH - 1 && ctx.input_scratch[MAP_STRIDE * y + x + 1] == cur + 1 {
                    stack.push((x + 1, y));
                }
                if x > 0 && ctx.input_scratch[MAP_STRIDE * y + x - 1] == cur + 1 {
                    stack.push((x - 1, y));
                }
                if y < MAP_WIDTH - 1 && ctx.input_scratch[MAP_STRIDE * (y + 1) + x] == cur + 1 {
                    stack.push((x, y + 1));
                }
                if y > 0 && ctx.input_scratch[MAP_STRIDE * (y - 1) + x] == cur + 1 {
                    stack.push((x, y - 1));
                }
            }
        }
    }
    Ok(sum as u64)
}
