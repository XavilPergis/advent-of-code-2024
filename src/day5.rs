use std::collections::{HashMap, HashSet};

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
}

// terrible memory layout lol
fn parse(ctx: &mut RunContext) -> eyre::Result<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let mut lines_iter = ctx.input.lines();

    let mut implication_map = HashMap::<u32, Vec<u32>>::new();
    let mut updates = vec![];
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let Some((l, r)) = line.split_once('|') else {
            eyre::bail!("invalid dependency rule");
        };
        let (l, r) = (l.parse()?, r.parse()?);
        implication_map.entry(r).or_default().push(l);
    }

    while let Some(line) = lines_iter.next() {
        updates.push(
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    ctx.mark_parse_complete();

    Ok((implication_map, updates))
}

fn part1(ctx: &mut RunContext) -> eyre::Result<()> {
    // A|B -> for each number N in update, find rules like A|N and check if A was already seen (how to discard non-matching rules?)

    let (implications, updates) = parse(ctx)?;

    let mut sum = 0;
    'outer: for update in &updates {
        let valid_elems = update.iter().copied().collect::<HashSet<_>>();
        let mut seen_elems = HashSet::<u32>::new();
        for &num in update {
            for implicated in implications[&num]
                .iter()
                .filter(|elem| valid_elems.contains(elem))
            {
                if !seen_elems.contains(implicated) {
                    continue 'outer;
                    // invalid!
                }
            }
            seen_elems.insert(num);
        }

        sum += update[(update.len() - 1) / 2];
    }

    println!("{sum}");

    Ok(())
}
