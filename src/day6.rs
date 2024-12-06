use std::ops::{Index, IndexMut};

use crate::{RunContext, RunnerRepository};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    // repo.add_variant("part2", part2);
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
    height: usize,
    data: Vec<T>,
}

impl<T> Vec2d<T> {
    fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            height,
            data,
        }
    }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Empty,
    Seen,
}

struct State {
    dir: Direction,
    pos: (isize, isize),
    width: usize,
    height: usize,
    map: Vec2d<Tile>,
}

impl State {
    fn inbounds(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
    fn is_wall(&self, pos: (isize, isize)) -> bool {
        match self.inbounds(pos) {
            true => self.map[cast_pos(pos)] == Tile::Wall,
            false => false,
        }
    }
}

fn cast_pos(pos: (isize, isize)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}

fn part1(ctx: &mut RunContext) -> eyre::Result<()> {
    let mut map = vec![];
    let mut pos = (0, 0);
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
                    pos = (xi as isize, yi as isize)
                }
                _ => eyre::bail!("unexpected char in input: '{}'", ch as char),
            }
        }
    }

    let mut state = State {
        dir: Direction::Up,
        pos,
        width,
        height,
        map: Vec2d::new(width, height, map),
    };

    while state.inbounds(state.pos) {
        assert_ne!(state.map[cast_pos(state.pos)], Tile::Wall);
        state.map[cast_pos(state.pos)] = Tile::Seen;

        let next_pos = match state.dir {
            Direction::Up => (state.pos.0, state.pos.1 - 1),
            Direction::Right => (state.pos.0 + 1, state.pos.1),
            Direction::Down => (state.pos.0, state.pos.1 + 1),
            Direction::Left => (state.pos.0 - 1, state.pos.1),
        };

        if state.is_wall(next_pos) {
            state.dir = match state.dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
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
    println!("{total}");

    for y in 0..state.map.height {
        for x in 0..state.map.width {
            print!(
                "{}",
                match state.map[(x, y)] {
                    Tile::Wall => '#',
                    Tile::Empty => ' ',
                    Tile::Seen => '.',
                }
            );
        }
        println!();
    }

    Ok(())
}
