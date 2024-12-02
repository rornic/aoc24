use aoc24::{Problem, run_problem};
use regex::Regex;

struct Day1 {}

fn parse_input(input: &str) -> [Vec<u32>; 2] {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let re = Regex::new(r"(\d+)\s*(\d+)").unwrap();
    for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    [left, right]
}

impl Problem for Day1 {
    fn part1(&self, input: &str) -> String {
        let [mut left, mut right] = parse_input(input);
        left.sort();
        right.sort();

        let result: u32 = left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
        format!("{}", result)
    }

    fn part2(&self, input: &str) -> String {
        let [left, right] = parse_input(input);

        let similarity_score: u32 = left.iter().map(|num| *num * right.iter().filter(|r| **r == *num).count() as u32).sum();
        format!("{}", similarity_score)
    }
}

fn main() {
    run_problem(Day1{}, "day1");
}
