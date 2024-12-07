use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn part1_verify(line: &[u32]) -> bool {
    assert!(line.len() >= 2);

    let dir = line[0].cmp(&line[1]);

    let mut prev = line[0];
    for &num in &line[1..] {
        if prev.cmp(&num) != dir {
            return false;
        }
        if prev == num || u32::abs_diff(prev, num) > 3 {
            return false;
        }
        prev = num;
    }

    true
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut safe_count = 0;
    let mut line_data = Vec::<u32>::new();
    for line in ctx.input.lines() {
        line_data.clear();
        for level in line.split_ascii_whitespace() {
            line_data.push(level.parse()?);
        }

        if part1_verify(&line_data) {
            safe_count += 1;
        }
    }

    Ok(safe_count as u64)
}

fn part2_verify(line: &[u32]) -> bool {
    assert!(line.len() >= 2);

    if !part1_verify(line) {
        let mut modified = Vec::with_capacity(line.len() - 1);
        for i in 0..line.len() {
            // construct a version of `line` with the value at index `i` removed.
            modified.clear();
            modified.extend_from_slice(line);
            modified.remove(i);
            // for j in 0..line.len() {
            //     if i != j {
            //         modified.push(line[j]);
            //     }
            // }

            if part1_verify(&modified) {
                return true;
            }
        }
        return false;
    }

    true
}

#[test]
fn test_day2_part2() {
    assert_eq!(part2_verify(&[7, 6, 4, 2, 1]), true); //: Safe without removing any level.
    assert_eq!(part2_verify(&[1, 2, 7, 8, 9]), false); //: Unsafe regardless of which level is removed.
    assert_eq!(part2_verify(&[9, 7, 6, 2, 1]), false); //: Unsafe regardless of which level is removed.
    assert_eq!(part2_verify(&[1, 3, 2, 4, 5]), true); //: Safe by removing the second level, 3.
    assert_eq!(part2_verify(&[8, 6, 4, 4, 1]), true); //: Safe by removing the third level, 4.
    assert_eq!(part2_verify(&[1, 3, 6, 7, 9]), true); //: Safe without removing any level.
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut safe_count = 0;
    let mut line_data = Vec::<u32>::new();
    for line in ctx.input.lines() {
        line_data.clear();
        for level in line.split_ascii_whitespace() {
            line_data.push(level.parse()?);
        }

        if part2_verify(&line_data) {
            safe_count += 1;
        }
    }

    Ok(safe_count as u64)
}
