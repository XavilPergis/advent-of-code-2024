use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn parse_number(input: &str, cur: &mut usize) -> eyre::Result<i64> {
    let begin = *cur;
    while input.as_bytes()[*cur].is_ascii_digit() {
        *cur += 1;
    }
    Ok(input[begin..*cur].parse::<i64>()?)
}

fn run_part(ctx: &mut RunContext, offset: i64) -> eyre::Result<impl Display> {
    let mut cur = 0;
    let mut sum = 0;

    while cur < ctx.input.len() {
        let ax = 10 * (ctx.input_scratch[cur + 12] - b'0') as i64
            + (ctx.input_scratch[cur + 13] - b'0') as i64;
        let ay = 10 * (ctx.input_scratch[cur + 18] - b'0') as i64
            + (ctx.input_scratch[cur + 19] - b'0') as i64;
        let bx = 10 * (ctx.input_scratch[cur + 33] - b'0') as i64
            + (ctx.input_scratch[cur + 34] - b'0') as i64;
        let by = 10 * (ctx.input_scratch[cur + 39] - b'0') as i64
            + (ctx.input_scratch[cur + 40] - b'0') as i64;
        cur += 51;

        while ctx.input_scratch[cur].is_ascii_digit() {
            cur += 1;
        }

        let px = offset + parse_number(ctx.input, &mut cur)?;
        cur += 4; // ", Y="
        let py = offset + parse_number(ctx.input, &mut cur)?;
        cur += 2; // "\n\n"

        // this is a matrix multiplication lol
        // `denom` is the scalar that you multiply a 2x2 matrix with when inverting.
        // the idea here is to transform points from XY space to AB space, which will tell us
        // exactly how many of each press we need to do.
        let denom = ax * by - bx * ay;
        let px_ab = px * by - py * bx;
        let py_ab = py * ax - px * ay;

        // if there's a remained, that means the target point is off-grid and therefore unreachable
        // (we'd need to do partial steps to reach it)
        if px_ab % denom == 0 && py_ab % denom == 0 {
            sum += 3 * (px_ab + py_ab) / denom;
        }
    }

    Ok(sum)
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    run_part(ctx, 0)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    run_part(ctx, 10000000000000)
}
