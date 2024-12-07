use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

fn solve_part1(test_value: u64, acc: u64, parts: &[u64]) -> bool {
    let (&head, rest) = match parts.split_first() {
        Some(res) => res,
        None => return test_value == acc,
    };
    solve_part1(test_value, acc + head, rest) || solve_part1(test_value, acc * head, rest)
}

fn part1(ctx: &mut RunContext) -> eyre::Result<()> {
    let mut total = 0;
    let mut parts = vec![];
    for line in ctx.input.lines() {
        let (test_value, rest) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<u64>()?;
        parts.clear();
        for ch in rest.split_ascii_whitespace() {
            parts.push(ch.parse::<u64>()?);
        }

        if solve_part1(test_value, parts[0], &parts[1..]) {
            total += test_value;
        }
    }
    println!("{total}");
    Ok(())
}
