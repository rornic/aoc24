use std::collections::{HashMap, HashSet};

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

        let left = left.into_iter().collect::<HashSet<u32>>();

        let mut counts = HashMap::<u32, u32>::new();
        for l in left.into_iter() {
            let count = right.iter().filter(|r| **r == l).count() as u32;
            counts.insert(l, count);
        }

        let similarity_score: u32 = counts.into_iter().map(|(num, count)| num * count).sum();
        format!("{}", similarity_score)
    }
}

fn main() {
    run_problem(Day1{}, "day1");
}
