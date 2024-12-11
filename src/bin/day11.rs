use std::collections::HashMap;

use aoc24::{run_problem, Problem};

struct Day11 {}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim_end()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn count_stones(stones: &[u64], steps: u64) -> u64 {
    let mut stone_counts: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        *stone_counts.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_stone_counts = HashMap::new();
        for (stone, count) in stone_counts {
            if stone == 0 {
                *new_stone_counts.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let (first, second) = stone_str.split_at(stone_str.len() / 2);
                *new_stone_counts.entry(first.parse().unwrap()).or_insert(0) += count;
                *new_stone_counts.entry(second.parse().unwrap()).or_insert(0) += count;
            } else {
                *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stone_counts = new_stone_counts;
    }

    stone_counts.into_values().sum()
}

impl Problem for Day11 {
    fn part1(&self, input: &str) -> String {
        let stones = parse_input(input);
        format!(
            "number of stones after 25 steps is {}",
            count_stones(&stones, 25)
        )
    }

    fn part2(&self, input: &str) -> String {
        let stones = parse_input(input);
        format!(
            "number of stones after 75 steps is {}",
            count_stones(&stones, 75)
        )
    }
}

fn main() {
    run_problem(Day11 {}, "day11");
}
