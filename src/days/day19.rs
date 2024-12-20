use ahash::{HashMap, HashMapExt};

use crate::prelude::*;

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

// white (w), blue (u), black (b), red (r), or green (g)

#[derive(Debug, Default)]
struct Node {
    is_terminal: bool,
    edges: [Option<Box<Node>>; 5],
}

fn colidx(ch: u8) -> usize {
    match ch {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        ch => panic!("invalid letter: {ch}"),
    }
}

impl Node {
    fn insert(&mut self, pattern: &[u8]) {
        let [head, tail @ ..] = pattern else {
            self.is_terminal = true;
            return;
        };
        match &mut self.edges[colidx(*head)] {
            Some(edge) => edge.insert(tail),
            edge @ None => {
                let mut child = Node::default();
                child.insert(tail);
                *edge = Some(Box::new(child));
            }
        }
    }
}

// fn walk_nodes(input: &[u8], root: &Node, node: &Node) -> bool {
//     todo!()
// }

// abcd
// abdc

// a -> b -> c -> (d)
//         ` d -> (c)

// abcdabdc
// * -a> . -b> . -c> . -d> #
//               -d> . -c> #

fn can_make_pattern(input: &[u8], node: &Node) -> bool {
    let mut cur_node = Some(node);
    let mut input = input;

    while let Some(cur) = cur_node {
        let [head, tail @ ..] = input else {
            return true;
        };
        let Some(next) = &cur.edges[colidx(*head)] else {
            break;
        };
        if next.is_terminal && can_make_pattern(tail, node) {
            return true;
        }
        input = tail;
        cur_node = Some(&next);
    }

    false
}

fn count_permutations(cache: &mut HashMap<usize, usize>, input: &[u8], offset: usize, node: &Node) -> usize {
    if let Some(&cached) = cache.get(&offset) {
        return cached;
    }

    if offset == input.len() {
        // cache.insert(offset, 1);
        return 1;
    }

    let mut cur_node = Some(node);
    let mut cur_offset = offset;

    let mut total = 0;
    while let Some(cur) = cur_node {
        let [head, ..] = &input[cur_offset..] else {
            return total;
        };
        let Some(next) = &cur.edges[colidx(*head)] else {
            break;
        };
        cur_offset += 1;
        if next.is_terminal {
            total += count_permutations(cache, input, cur_offset, node);
        }
        cur_node = Some(&next);
    }

    cache.insert(offset, total);

    total
}

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut node = Node::default();

    let mut lines = ctx.input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        for pattern in line.split(", ") {
            node.insert(pattern.as_bytes());
        }
    }

    let mut res = 0;
    while let Some(line) = lines.next() {
        if can_make_pattern(line.as_bytes(), &node) {
            res += 1;
        }
    }

    Ok(res)
}

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut node = Node::default();

    let mut lines = ctx.input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        for pattern in line.split(", ") {
            node.insert(pattern.as_bytes());
        }
    }

    let mut cache = HashMap::new();
    let mut res = 0;
    while let Some(line) = lines.next() {
        cache.clear();
        res += count_permutations(&mut cache, line.as_bytes(), 0, &node);
    }

    Ok(res)
}
