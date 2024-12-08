use std::{
    collections::HashSet,
    ops::{Add, Index, IndexMut, Sub},
};

use crate::{
    bitset::{self, FixedBitset},
    RunContext, RunnerRepository,
};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part1_bitset", part1_bitset);
    repo.add_variant("part2", part2);
    repo.add_variant("part2_bitset", part2_bitset);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Vec2d<T> {
    width: usize,
    _height: usize,
    data: Vec<T>,
}

impl<T> Vec2d<T> {
    fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            _height: height,
            data,
        }
    }
    // fn broadcast(width: usize, height: usize, value: T) -> Self
    // where
    //     T: Clone,
    // {
    //     Self {
    //         width,
    //         _height: height,
    //         data: vec![value; width * height],
    //     }
    // }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[self.width * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.width * y + x]
    }
}

impl<T> Index<Vec2> for Vec2d<T> {
    type Output = T;
    fn index(&self, Vec2 { x, y }: Vec2) -> &Self::Output {
        &self.data[self.width * y as usize + x as usize]
    }
}

impl<T> IndexMut<Vec2> for Vec2d<T> {
    fn index_mut(&mut self, Vec2 { x, y }: Vec2) -> &mut Self::Output {
        &mut self.data[self.width * y as usize + x as usize]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Empty,
    Seen,
}

struct State {
    dir: Direction,
    pos: Vec2,
    width: usize,
    height: usize,
    map: Vec2d<Tile>,
}

impl State {
    fn inbounds(&self, Vec2 { x, y }: Vec2) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
    fn is_wall(&self, pos: Vec2) -> bool {
        match self.inbounds(pos) {
            true => self.map[pos] == Tile::Wall,
            false => false,
        }
    }
}

// `dir` is the direction the guard would be heading towards
// fn would_loop(state: &State, pos: (isize, isize), dir: Direction) -> bool {
//     let mut seen = HashSet::new();

//     todo!()
// }

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

// fn vec2(x: usize, y: usize) -> Vec2 {
//     Vec2 {
//         x: x as isize,
//         y: y as isize,
//     }
// }

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn offset_from_dir(dir: Direction) -> Vec2 {
    match dir {
        Direction::Up => Vec2 { x: 0, y: -1 },
        Direction::Right => Vec2 { x: 1, y: 0 },
        Direction::Down => Vec2 { x: 0, y: 1 },
        Direction::Left => Vec2 { x: -1, y: 0 },
    }
}

fn rotate_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn parse(ctx: &mut RunContext) -> eyre::Result<State> {
    let mut map = vec![];
    let mut pos = Vec2 { x: 0, y: 0 };
    let mut width = 0;
    let mut height = 0;
    for (yi, line) in ctx.input.lines().enumerate() {
        if line.trim().is_empty() {
            break;
        }
        width = line.trim().len();
        height += 1;
        for (xi, ch) in line.trim().bytes().enumerate() {
            match ch {
                b'.' => map.push(Tile::Empty),
                b'#' => map.push(Tile::Wall),
                b'^' => {
                    map.push(Tile::Empty);
                    pos = Vec2 {
                        x: xi as isize,
                        y: yi as isize,
                    }
                }
                _ => eyre::bail!("unexpected char in input: '{}'", ch as char),
            }
        }
    }

    ctx.mark_parse_complete();

    Ok(State {
        dir: Direction::Up,
        pos,
        width,
        height,
        map: Vec2d::new(width, height, map),
    })
}

fn part1(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut state = parse(ctx)?;

    while state.inbounds(state.pos) {
        assert_ne!(state.map[state.pos], Tile::Wall);
        state.map[state.pos] = Tile::Seen;

        let next_pos = state.pos + offset_from_dir(state.dir);
        if state.is_wall(next_pos) {
            state.dir = rotate_dir(state.dir);
        } else {
            state.pos = next_pos;
        }
    }

    let total = state
        .map
        .data
        .iter()
        .filter(|&&tile| tile == Tile::Seen)
        .count();

    // for y in 0..state.map.height {
    //     for x in 0..state.map.width {
    //         print!(
    //             "{}",
    //             match state.map[(x, y)] {
    //                 Tile::Wall => '#',
    //                 Tile::Empty => ' ',
    //                 Tile::Seen => '.',
    //             }
    //         );
    //     }
    //     println!();
    // }

    Ok(total as u64)
}

const BOARD_LEN: usize = 130;
const BOARD_AREA: usize = 130 * 130;
const HI64: u64 = 1u64 << 63;

fn part1_bitset(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut walls = FixedBitset::new(BOARD_AREA);
    let mut visited = FixedBitset::new(BOARD_AREA);
    let mut x = 0;
    let mut y = 0;

    let mut acc = 0;
    let mut base = 0;
    let mut n = 0;
    for &ch in ctx.input.as_bytes() {
        if ch == b'\r' || ch == b'\n' {
            continue;
        }
        acc |= (HI64 * (ch == b'#') as u64) >> (n & 63);
        n += 1;
        if n & 63 == 0 {
            walls.set_many(base, acc);
            acc = 0;
            base = n;
        }
    }

    let ix = ctx
        .input
        .as_bytes()
        .iter()
        .position(|&ch| ch == b'^')
        .unwrap();
    y = ix / (BOARD_LEN + 1);
    x = ix % (BOARD_LEN + 1);

    'outer: loop {
        loop {
            visited.set(BOARD_LEN * y + x);
            if y == 0 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * (y - 1) + x) {
                break;
            }
            y -= 1;
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if x == BOARD_LEN - 1 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * y + x + 1) {
                break;
            }
            x += 1;
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if y == BOARD_LEN - 1 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * (y + 1) + x) {
                break;
            }
            y += 1;
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if x == 0 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * y + x - 1) {
                break;
            }
            x -= 1;
        }
    }

    // for y in 0..BOARD_LEN {
    //     for x in 0..BOARD_LEN {
    //         print!(
    //             "{}",
    //             match (walls.get(BOARD_LEN * y + x), visited.get(BOARD_LEN * y + x)) {
    //                 (true, true) => '?',
    //                 (true, false) => '#',
    //                 (false, true) => '.',
    //                 (false, false) => ' ',
    //             }
    //         );
    //     }
    //     println!();
    // }

    let total = visited.count_ones();
    Ok(total as u64)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut state = parse(ctx)?;

    let start_pos = state.pos;
    let start_dir = state.dir;

    let mut canditates = Vec::new();
    while state.inbounds(state.pos) {
        assert_ne!(state.map[state.pos], Tile::Wall);
        state.map[state.pos] = Tile::Seen;

        let next_pos = state.pos + offset_from_dir(state.dir);
        if state.is_wall(next_pos) {
            state.dir = rotate_dir(state.dir);
        } else {
            canditates.push(next_pos);
            state.pos = next_pos;
        }
    }

    let mut obstacles = HashSet::new();
    let mut total = 0;
    let mut seen = HashSet::new();
    for &obstacle_pos in &canditates {
        state.dir = start_dir;
        state.pos = start_pos;
        if !state.inbounds(obstacle_pos) {
            continue;
        }
        if obstacle_pos == state.pos {
            continue;
        }
        state.map[obstacle_pos] = Tile::Wall;

        seen.clear();

        while state.inbounds(state.pos) {
            assert_ne!(state.map[state.pos], Tile::Wall);

            if !seen.insert((state.dir, state.pos)) {
                // we found a loop!
                if obstacles.insert(obstacle_pos) {
                    total += 1;
                }
                break;
            }

            let next_pos = state.pos + offset_from_dir(state.dir);
            if state.is_wall(next_pos) {
                state.dir = rotate_dir(state.dir);
            } else {
                state.pos = next_pos;
            }
        }

        // undo temporary obstacle placement. We don't produce any candidates that have walls in front in the first place, so this is fine.
        state.map[obstacle_pos] = Tile::Empty;
    }

    // for y in 0..state.map.height {
    //     for x in 0..state.map.width {
    //         print!(
    //             "{}",
    //             match (obstacles.contains(&vec2(x, y)), state.map[(x, y)]) {
    //                 (true, Tile::Wall) => '?',
    //                 (false, Tile::Wall) => '#',
    //                 (true, _) => 'O',
    //                 (false, _) => ' ',
    //             }
    //         );
    //     }
    //     println!();
    // }

    Ok(total as u64)
}

fn part2_bitset(ctx: &mut RunContext) -> eyre::Result<u64> {
    let mut walls = FixedBitset::new(BOARD_AREA);
    let mut walls_t = FixedBitset::new(BOARD_AREA);
    let mut visited = FixedBitset::new(BOARD_AREA);
    let mut x = 0;
    let mut y = 0;

    let mut acc = 0;
    let mut base = 0;
    let mut n = 0;
    for &ch in ctx.input.as_bytes() {
        if ch == b'\r' || ch == b'\n' {
            continue;
        }
        acc |= (HI64 * (ch == b'#') as u64) >> (n & 63);
        n += 1;
        if n & 63 == 0 {
            walls.set_many(base, acc);
            acc = 0;
            base = n;
        }
    }

    let ix = ctx
        .input
        .as_bytes()
        .iter()
        .position(|&ch| ch == b'^')
        .unwrap();
    let start_y = ix / (BOARD_LEN + 1);
    let start_x = ix % (BOARD_LEN + 1);

    x = start_x;
    y = start_y;

    let mut candidates = Vec::with_capacity(BOARD_AREA);

    'outer: loop {
        loop {
            visited.set(BOARD_LEN * y + x);
            if y == 0 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * (y - 1) + x) {
                break;
            }
            y -= 1;
            if !visited.get(BOARD_LEN * y + x) {
                candidates.push((x, y));
            }
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if x == BOARD_LEN - 1 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * y + x + 1) {
                break;
            }
            x += 1;
            if !visited.get(BOARD_LEN * y + x) {
                candidates.push((x, y));
            }
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if y == BOARD_LEN - 1 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * (y + 1) + x) {
                break;
            }
            y += 1;
            if !visited.get(BOARD_LEN * y + x) {
                candidates.push((x, y));
            }
        }
        loop {
            visited.set(BOARD_LEN * y + x);
            if x == 0 {
                break 'outer;
            }
            if walls.get(BOARD_LEN * y + x - 1) {
                break;
            }
            x -= 1;
            if !visited.get(BOARD_LEN * y + x) {
                candidates.push((x, y));
            }
        }
    }

    let mut total = 0;
    let mut visited_u = FixedBitset::new(BOARD_AREA);
    for &candidate in &candidates {
        visited_u.clear_all();
        x = start_x;
        y = start_y;
        walls.set(BOARD_LEN * candidate.1 + candidate.0);

        'outer: loop {
            loop {
                // detect cycles. if it's a cycle, then it doesnt matter when exactly we detect the cycle,
                // just that we *do*. This allows us to only check for cycles on one axis and not waste
                // time updating/querying/clearing visited sets for each direction.
                if visited_u.get(BOARD_LEN * y + x) {
                    total += 1;
                    break 'outer;
                }
                visited_u.set(BOARD_LEN * y + x);
                if y == 0 {
                    break 'outer;
                }
                if walls.get(BOARD_LEN * (y - 1) + x) {
                    break;
                }
                y -= 1;
            }
            loop {
                if x == BOARD_LEN - 1 {
                    break 'outer;
                }
                if walls.get(BOARD_LEN * y + x + 1) {
                    break;
                }
                x += 1;
            }
            loop {
                if y == BOARD_LEN - 1 {
                    break 'outer;
                }
                if walls.get(BOARD_LEN * (y + 1) + x) {
                    break;
                }
                y += 1;
            }
            loop {
                if x == 0 {
                    break 'outer;
                }
                if walls.get(BOARD_LEN * y + x - 1) {
                    break;
                }
                x -= 1;
            }
        }

        walls.clear(BOARD_LEN * candidate.1 + candidate.0);
    }

    Ok(total as u64)
}
