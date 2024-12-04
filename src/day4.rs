use std::ops::Index;

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part1_split", part1_split);
    repo.add_variant("part2", part2);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

struct Board {
    width: usize,
    height: usize,
    letters: Box<[Letter]>,
}

fn parse_board(ctx: &mut RunContext) -> eyre::Result<Board> {
    let mut res = Vec::new();
    let mut width = None;
    let mut height = 0;

    for line in ctx.input.lines() {
        if line.is_empty() {
            continue;
        }
        height += 1;
        width = Some(line.trim().len());
        for ch in line.trim().chars() {
            match ch {
                'X' => res.push(Letter::X),
                'M' => res.push(Letter::M),
                'A' => res.push(Letter::A),
                'S' => res.push(Letter::S),
                _ => eyre::bail!("invalid char: {ch}"),
            };
        }
    }

    ctx.mark_parse_complete();

    Ok(Board {
        width: width.unwrap(),
        height,
        letters: res.into_boxed_slice(),
    })
}

impl Index<(usize, usize)> for Board {
    type Output = Letter;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.letters[self.width * y + x]
    }
}

fn check_surrounding(board: &Board, x: usize, y: usize) -> usize {
    let mut res = 0;

    if x + 4 <= board.width {
        if board[(x, y)] == Letter::X
            && board[(x + 1, y)] == Letter::M
            && board[(x + 2, y)] == Letter::A
            && board[(x + 3, y)] == Letter::S
        {
            res += 1;
        }
        if board[(x, y)] == Letter::S
            && board[(x + 1, y)] == Letter::A
            && board[(x + 2, y)] == Letter::M
            && board[(x + 3, y)] == Letter::X
        {
            res += 1;
        }
    }

    if y + 4 <= board.height {
        if board[(x, y)] == Letter::X
            && board[(x, y + 1)] == Letter::M
            && board[(x, y + 2)] == Letter::A
            && board[(x, y + 3)] == Letter::S
        {
            res += 1;
        }
        if board[(x, y)] == Letter::S
            && board[(x, y + 1)] == Letter::A
            && board[(x, y + 2)] == Letter::M
            && board[(x, y + 3)] == Letter::X
        {
            res += 1;
        }
    }

    if x + 4 <= board.width && y + 4 <= board.height {
        if board[(x, y)] == Letter::X
            && board[(x + 1, y + 1)] == Letter::M
            && board[(x + 2, y + 2)] == Letter::A
            && board[(x + 3, y + 3)] == Letter::S
        {
            res += 1;
        }
        if board[(x, y)] == Letter::S
            && board[(x + 1, y + 1)] == Letter::A
            && board[(x + 2, y + 2)] == Letter::M
            && board[(x + 3, y + 3)] == Letter::X
        {
            res += 1;
        }
    }

    if x + 4 <= board.width && y >= 3 {
        if board[(x, y)] == Letter::X
            && board[(x + 1, y - 1)] == Letter::M
            && board[(x + 2, y - 2)] == Letter::A
            && board[(x + 3, y - 3)] == Letter::S
        {
            res += 1;
        }
        if board[(x, y)] == Letter::S
            && board[(x + 1, y - 1)] == Letter::A
            && board[(x + 2, y - 2)] == Letter::M
            && board[(x + 3, y - 3)] == Letter::X
        {
            res += 1;
        }
    }
    res
}

fn part1(ctx: &mut RunContext) -> eyre::Result<()> {
    let board = parse_board(ctx)?;

    let mut res = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            res += check_surrounding(&board, x, y);
        }
    }

    println!("{res}");

    Ok(())
}

fn part1_split(ctx: &mut RunContext) -> eyre::Result<()> {
    let board = parse_board(ctx)?;

    let mut res = 0;

    const XMAS: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];
    const SAMX: [Letter; 4] = [Letter::S, Letter::A, Letter::M, Letter::X];

    // right
    for y in 0..board.height {
        for x in 0..board.width - 3 {
            let line = [
                board[(x, y)],
                board[(x + 1, y)],
                board[(x + 2, y)],
                board[(x + 3, y)],
            ];
            if line == XMAS || line == SAMX {
                res += 1;
            }
        }
    }

    // down
    for y in 0..board.height - 3 {
        for x in 0..board.width {
            let line = [
                board[(x, y)],
                board[(x, y + 1)],
                board[(x, y + 2)],
                board[(x, y + 3)],
            ];
            if line == XMAS || line == SAMX {
                res += 1;
            }
        }
    }

    // down-right
    for y in 0..board.height - 3 {
        for x in 0..board.width - 3 {
            let line = [
                board[(x, y)],
                board[(x + 1, y + 1)],
                board[(x + 2, y + 2)],
                board[(x + 3, y + 3)],
            ];
            if line == XMAS || line == SAMX {
                res += 1;
            }
        }
    }

    // up-right
    for y in 3..board.height {
        for x in 0..board.width - 3 {
            let line = [
                board[(x, y)],
                board[(x + 1, y - 1)],
                board[(x + 2, y - 2)],
                board[(x + 3, y - 3)],
            ];
            if line == XMAS || line == SAMX {
                res += 1;
            }
        }
    }

    println!("{res}");

    Ok(())
}

fn check_surrounding_part2(board: &Board, x: usize, y: usize) -> usize {
    let mut res = 0;

    if board[(x, y)] == Letter::A && x > 0 && y > 0 && x < board.width - 1 && y < board.height - 1 {
        let nn = board[(x - 1, y - 1)];
        let np = board[(x - 1, y + 1)];
        let pn = board[(x + 1, y - 1)];
        let pp = board[(x + 1, y + 1)];

        let cross1 = nn == Letter::M && pp == Letter::S || nn == Letter::S && pp == Letter::M;
        let cross2 = np == Letter::M && pn == Letter::S || np == Letter::S && pn == Letter::M;

        if cross1 && cross2 {
            res += 1;
        }
    }

    res
}

fn part2(ctx: &mut RunContext) -> eyre::Result<()> {
    let board = parse_board(ctx)?;

    let mut res = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            res += check_surrounding_part2(&board, x, y);
        }
    }

    println!("{res}");

    Ok(())
}
