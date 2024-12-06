use std::collections::{HashMap, HashSet};

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn part1(ctx: &mut RunContext) -> eyre::Result<()> {
    // A|B -> for each number N in update, find rules like A|N and check if A was already seen (how to discard non-matching rules?)

    let mut lines_iter = ctx.input.lines();

    let mut implications = HashMap::<u32, Vec<u32>>::new();
    let mut updates = vec![];
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let Some((l, r)) = line.split_once('|') else {
            eyre::bail!("invalid dependency rule");
        };
        let (l, r) = (l.parse()?, r.parse()?);
        implications.entry(r).or_default().push(l);
    }

    while let Some(line) = lines_iter.next() {
        updates.push(
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    ctx.mark_parse_complete();

    let mut sum = 0;
    let mut valid_elems = HashSet::<u32>::new();
    let mut seen_elems = HashSet::<u32>::new();
    'outer: for update in &updates {
        valid_elems.clear();
        seen_elems.clear();
        valid_elems.extend(update.iter().copied());
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

fn toposort(
    out: &mut Vec<u32>,
    seen: &mut HashSet<u32>,
    current: u32,
    valid: &HashSet<u32>,
    graph: &HashMap<u32, Vec<u32>>,
) {
    if !seen.insert(current) {
        return;
    }
    for &outgoing in graph[&current].iter().filter(|elem| valid.contains(elem)) {
        toposort(out, seen, outgoing, valid, graph);
    }
    out.push(current);
}

fn part2(ctx: &mut RunContext) -> eyre::Result<()> {
    let mut lines_iter = ctx.input.lines();

    let mut ordered_after = HashMap::<u32, Vec<u32>>::new();
    let mut ordered_before = HashMap::<u32, Vec<u32>>::new();
    let mut updates = vec![];
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let Some((l, r)) = line.split_once('|') else {
            eyre::bail!("invalid dependency rule");
        };
        let (l, r) = (l.parse()?, r.parse()?);
        ordered_after.entry(r).or_default().push(l);
        ordered_before.entry(l).or_default().push(r);
    }

    while let Some(line) = lines_iter.next() {
        updates.push(
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    ctx.mark_parse_complete();

    let mut sum = 0;
    'outer: for update in &updates {
        let valid_elems = update.iter().copied().collect::<HashSet<_>>();
        let mut seen_elems = HashSet::<u32>::new();
        for &num in update {
            for implicated in ordered_after[&num]
                .iter()
                .filter(|elem| valid_elems.contains(elem))
            {
                if !seen_elems.contains(implicated) {
                    // find number in update that is not ordered before anything, this is our root. (indegree=0)
                    let root = update.iter().find(|elem| {
                        ordered_before[&elem]
                            .iter()
                            .filter(|elem2| valid_elems.contains(elem2))
                            .next()
                            .is_none()
                    }).unwrap();

                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    toposort(&mut out, &mut seen, *root, &valid_elems, &ordered_after);

                    sum += out[(out.len() - 1) / 2];

                    continue 'outer;
                    // invalid!
                }
            }
            seen_elems.insert(num);
        }
    }

    println!("{sum}");

    Ok(())
}
