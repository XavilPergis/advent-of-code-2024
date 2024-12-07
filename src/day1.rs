use std::collections::HashMap;

use eyre::OptionExt;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse(ctx: &mut RunContext) -> eyre::Result<Lists> {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    for line in ctx.input.lines().filter(|line| !line.is_empty()) {
        let (left, right) = line.split_once("   ").ok_or_eyre("failed to split line")?;
        left_list.push(left.parse()?);
        right_list.push(right.parse()?);
    }

    ctx.mark_parse_complete();

    Ok(Lists {
        left: left_list,
        right: right_list,
    })
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut input = parse(ctx)?;

    input.left.sort_unstable();
    input.right.sort_unstable();

    let mut total_dist = 0;
    for (&left, &right) in input.left.iter().zip(&input.right) {
        total_dist += i32::abs_diff(left, right);
    }

    Ok(total_dist as u64)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let input = parse(ctx)?;

    let mut freq = HashMap::<i32, u32>::new();
    for &id in &input.right {
        *freq.entry(id).or_insert(0) += 1;
    }

    let mut score = 0;
    for &id in &input.left {
        score += id * freq.get(&id).copied().unwrap_or_default() as i32;
    }

    Ok(score as u64)
}
