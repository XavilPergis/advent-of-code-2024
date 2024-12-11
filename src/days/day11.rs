use std::collections::HashMap;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    let mut k = 0;
    let mut t = n;
    while t > 0 {
        t /= 10;
        k += 1;
    }

    if k % 2 != 0 {
        return None;
    }

    let p = 10u64.pow(k / 2);
    Some((n / p, n % p))
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut front = vec![];
    let mut back = vec![];

    let mut sum = 0;
    for num in ctx.input.split_whitespace() {
        front.clear();
        back.clear();
        front.push(num.parse::<u64>()?);
        for _ in 0..40 {
            // println!("{front:?}");
            for &num in &front {
                if num == 0 {
                    back.push(1);
                } else if let Some((l, r)) = split_digits(num) {
                    back.push(l);
                    back.push(r);
                } else {
                    back.push(num * 2024);
                }
            }
            front.clear();
            std::mem::swap(&mut front, &mut back);
        }
        sum += front.len();
    }

    Ok(sum as u64)
}

fn seq_count(cache: &mut HashMap<(u64, u8), u64>, num: u64, depth: u8) -> u64 {
    if let Some(&cached) = cache.get(&(num, depth)) {
        return cached;
    }

    let res = if depth == 75 {
        // base case
        1
    } else if num == 0 {
        seq_count(cache, 1, depth + 1)
    } else if let Some((l, r)) = split_digits(num) {
        seq_count(cache, l, depth + 1) + seq_count(cache, r, depth + 1)
    } else {
        seq_count(cache, num * 2024, depth + 1)
    };

    cache.insert((num, depth), res);
    res
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for num in ctx.input.split_whitespace() {
        sum += seq_count(&mut cache, num.parse::<u64>()?, 0);
    }
    Ok(sum)
}
