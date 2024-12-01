mod day1;

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    day1::part2(input);
}
