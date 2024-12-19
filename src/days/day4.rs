use std::ops::Index;

use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part1_split", part1_split);
    repo.add_variant("part2", part2);
}

struct Board {
    width: usize,
    height: usize,
    letters: Box<[u8]>,
}

fn parse_board(ctx: &mut RunContext) -> eyre::Result<Board> {
    let mut res = Vec::new();
    let mut width = None;
    let mut height = 0;
    let mut cur_width = 0;
    for &ch in ctx.input.as_bytes() {
        match ch {
            b'\r' => {}
            b'\n' => {
                width = Some(cur_width);
                height += 1;
                cur_width = 0;
            }
            _ => {
                res.push(ch);
                cur_width += 1;
            }
        }
    }

    // for line in ctx.input.lines() {
    //     if line.is_empty() {
    //         continue;
    //     }
    //     height += 1;
    //     width = Some(line.trim().len());
    //     res.extend_from_slice(line.trim().as_bytes());
    // }

    ctx.mark_parse_complete();

    Ok(Board {
        width: width.unwrap(),
        height,
        letters: res.into_boxed_slice(),
    })
}

impl Index<(usize, usize)> for Board {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.letters[self.width * y + x]
    }
}

fn check_surrounding(board: &Board, x: usize, y: usize) -> usize {
    let mut res = 0;

    if x + 4 <= board.width {
        if board[(x, y)] == b'X'
            && board[(x + 1, y)] == b'M'
            && board[(x + 2, y)] == b'A'
            && board[(x + 3, y)] == b'S'
        {
            res += 1;
        }
        if board[(x, y)] == b'S'
            && board[(x + 1, y)] == b'A'
            && board[(x + 2, y)] == b'M'
            && board[(x + 3, y)] == b'X'
        {
            res += 1;
        }
    }

    if y + 4 <= board.height {
        if board[(x, y)] == b'X'
            && board[(x, y + 1)] == b'M'
            && board[(x, y + 2)] == b'A'
            && board[(x, y + 3)] == b'S'
        {
            res += 1;
        }
        if board[(x, y)] == b'S'
            && board[(x, y + 1)] == b'A'
            && board[(x, y + 2)] == b'M'
            && board[(x, y + 3)] == b'X'
        {
            res += 1;
        }
    }

    if x + 4 <= board.width && y + 4 <= board.height {
        if board[(x, y)] == b'X'
            && board[(x + 1, y + 1)] == b'M'
            && board[(x + 2, y + 2)] == b'A'
            && board[(x + 3, y + 3)] == b'S'
        {
            res += 1;
        }
        if board[(x, y)] == b'S'
            && board[(x + 1, y + 1)] == b'A'
            && board[(x + 2, y + 2)] == b'M'
            && board[(x + 3, y + 3)] == b'X'
        {
            res += 1;
        }
    }

    if x + 4 <= board.width && y >= 3 {
        if board[(x, y)] == b'X'
            && board[(x + 1, y - 1)] == b'M'
            && board[(x + 2, y - 2)] == b'A'
            && board[(x + 3, y - 3)] == b'S'
        {
            res += 1;
        }
        if board[(x, y)] == b'S'
            && board[(x + 1, y - 1)] == b'A'
            && board[(x + 2, y - 2)] == b'M'
            && board[(x + 3, y - 3)] == b'X'
        {
            res += 1;
        }
    }
    res
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let board = parse_board(ctx)?;

    let mut res = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            res += check_surrounding(&board, x, y);
        }
    }

    Ok(res)
}

fn part1_split(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let board = parse_board(ctx)?;

    let mut res = 0;

    const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
    const SAMX: [u8; 4] = [b'S', b'A', b'M', b'X'];

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

    Ok(res)
}

fn check_surrounding_part2(board: &Board, x: usize, y: usize) -> usize {
    let mut res = 0;

    if board[(x, y)] == b'A' && x > 0 && y > 0 && x < board.width - 1 && y < board.height - 1 {
        let nn = board[(x - 1, y - 1)];
        let np = board[(x - 1, y + 1)];
        let pn = board[(x + 1, y - 1)];
        let pp = board[(x + 1, y + 1)];

        let cross1 = nn == b'M' && pp == b'S' || nn == b'S' && pp == b'M';
        let cross2 = np == b'M' && pn == b'S' || np == b'S' && pn == b'M';

        if cross1 && cross2 {
            res += 1;
        }
    }

    res
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let board = parse_board(ctx)?;

    let mut res = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            res += check_surrounding_part2(&board, x, y);
        }
    }

    Ok(res)
}
