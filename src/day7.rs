use std::{num::NonZeroU64, sync::atomic::AtomicU64};

use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
    repo.add_variant("part2_parallel", part2_parallel);
}

fn solve_part1(test_value: u64, acc: u64, parts: &[u64]) -> bool {
    if acc > test_value {
        return false;
    }
    let (&head, tail) = match parts.split_first() {
        Some(res) => res,
        None => return test_value == acc,
    };
    solve_part1(test_value, acc + head, tail) || solve_part1(test_value, acc * head, tail)
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut total = 0;
    let mut parts = vec![];
    for line in ctx.input.lines() {
        let (test_value, tail) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<u64>()?;
        parts.clear();
        for ch in tail.split_ascii_whitespace() {
            parts.push(ch.parse::<u64>()?);
        }

        if solve_part1(test_value, parts[0], &parts[1..]) {
            total += test_value;
        }
    }
    Ok(total)
}

fn concat(l: u64, r: u64) -> u64 {
    debug_assert_ne!(l, 0);
    debug_assert_ne!(r, 0);
    let mut res = l;
    let mut n = r;
    while n > 0 {
        res *= 10;
        n /= 10;
    }
    res + r
}

fn solve_part2(test_value: u64, acc: u64, parts: &[u64]) -> bool {
    if acc > test_value {
        return false;
    }
    let Some((&head, tail)) = parts.split_first() else {
        return test_value == acc;
    };
    solve_part2(test_value, acc + head, tail)
        || solve_part2(test_value, acc * head, tail)
        || solve_part2(test_value, concat(acc, head), tail)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut total = 0;
    let mut parts = vec![];
    for line in ctx.input.lines() {
        let (test_value, tail) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<u64>()?;
        parts.clear();
        for ch in tail.split_ascii_whitespace() {
            parts.push(ch.parse::<u64>()?);
        }

        if solve_part2(test_value, parts[0], &parts[1..]) {
            total += test_value;
        }
    }
    Ok(total)
}

fn part2_parallel(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut total = AtomicU64::new(0);
    // let mut parts = vec![];
    ctx.input.par_lines().try_for_each_init(
        || vec![],
        |parts, line| {
            let (test_value, tail) = line.split_once(": ").unwrap();
            let test_value = test_value.parse::<u64>()?;
            parts.clear();
            for ch in tail.split_ascii_whitespace() {
                parts.push(ch.parse::<u64>()?);
            }

            if solve_part2(test_value, parts[0], &parts[1..]) {
                // total += test_value;
                total.fetch_add(test_value, std::sync::atomic::Ordering::Relaxed);
            }

            Ok::<_, eyre::Report>(())
        },
    )?;
    let total = *total.get_mut();
    Ok(total)
}
