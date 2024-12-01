use std::collections::HashMap;

struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse(input: String) -> Lists {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    Lists {
        left: left_list,
        right: right_list,
    }
}

pub fn part1(input: String) {
    let mut lists = parse(input);

    lists.left.sort_unstable();
    lists.right.sort_unstable();

    let mut total_dist = 0;
    for (&left, &right) in lists.left.iter().zip(&lists.right) {
        total_dist += i32::abs_diff(left, right);
    }

    println!("{total_dist}");
}

pub fn part2(input: String) {
    let mut lists = parse(input);

    let mut freq = HashMap::<i32, u32>::new();
    for &id in &lists.right {
        *freq.entry(id).or_insert(0) += 1;
    }

    let mut score = 0;
    for &id in &lists.left {
        score += id * freq.get(&id).copied().unwrap_or_default() as i32;
    }

    println!("{score}");
}
