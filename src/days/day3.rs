use std::fmt::Display;

use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let regex = regex::Regex::new(r#"mul\(([0-9]{0,3}),([0-9]{0,3})\)"#).unwrap();
    let mut sum = 0;
    for matched in regex.captures_iter(ctx.input) {
        let (_, [l, r]) = matched.extract();
        sum += l.parse::<u32>()? * r.parse::<u32>()?;
    }
    Ok(sum)
}

fn eat_str(src: &mut &str, str: &str) -> bool {
    if src.starts_with(str) {
        *src = &src[str.len()..];
        true
    } else {
        false
    }
}

fn eat_digit(src: &mut &str) -> Option<u32> {
    let ch = src.chars().next()?;
    if ch.is_ascii_digit() {
        *src = &src[char::len_utf8(ch)..];
        Some((ch as u8 - b'0') as u32)
    } else {
        None
    }
}

fn eat_digits(src: &mut &str) -> Option<u32> {
    let mut res = eat_digit(src)?;
    while let Some(digit) = eat_digit(src) {
        res *= 10;
        res += digit;
    }
    Some(res)
}

fn parse_mul(src: &mut &str) -> Option<u32> {
    let a = eat_digits(src)?;
    eat_str(src, ",").then_some(())?;
    let b = eat_digits(src)?;
    eat_str(src, ")").then_some(())?;
    Some(a * b)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut src = ctx.input;

    let mut sum = 0;
    let mut mul_enabled = true;
    while !src.is_empty() {
        if eat_str(&mut src, "do()") {
            mul_enabled = true;
        } else if eat_str(&mut src, "don't()") {
            mul_enabled = false;
        } else if mul_enabled && eat_str(&mut src, "mul(") {
            if let Some(res) = parse_mul(&mut src) {
                sum += res;
            }
        } else {
            match src.chars().next() {
                Some(ch) => src = &src[char::len_utf8(ch)..],
                _ => break,
            }
        }
    }

    Ok(sum)
}
