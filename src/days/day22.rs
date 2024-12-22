use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn step_prng(state: u32) -> u32 {
    let mut state = state;
    state ^= state << 6;
    state &= 0xffffff;
    state ^= state >> 5;
    state &= 0xffffff;
    state ^= state << 11;
    state &= 0xffffff;
    state
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut res = 0;
    for line in ctx.input.lines() {
        let num = line.parse::<u32>()?;

        let mut state = num;
        for _ in 0..2000 {
            state = step_prng(state);
        }

        res += state as u64;
    }

    Ok(res)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut seq_prices = HashMap::new();
    let mut seen_per_buyer = HashSet::new();
    for line in ctx.input.lines() {
        seen_per_buyer.clear();
        let num = line.parse::<u32>()?;

        let mut prev = num;
        let mut state = step_prng(prev);
        let mut seq = [0, 0, 0, 0];
        for i in 0..1999 {
            let delta = (state % 10) as i8 - (prev % 10) as i8;
            seq[0] = seq[1];
            seq[1] = seq[2];
            seq[2] = seq[3];
            seq[3] = delta;

            if i >= 3 && seen_per_buyer.insert(seq) {
                *seq_prices.entry(seq).or_insert(0) += state % 10;
            }

            prev = state;
            state = step_prng(state);
        }
    }

    let max_price = *seq_prices.values().max().unwrap();

    Ok(max_price as u64)
}
