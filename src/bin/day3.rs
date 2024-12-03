use aoc24::{run_problem, Problem};
use regex::Regex;

struct Day3 {}

struct Mul {
    x: u32,
    y: u32,
}

fn parse_input_dos_donts(input: &str) -> Vec<Mul> {
    let mut muls = vec![];

    let mut skipping = false;
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+,\d+)\)").unwrap();
    for (_, [instruction]) in re.captures_iter(input).map(|c| c.extract()) {
        if instruction == "don't()" {
            skipping = true;
            continue;
        } else if instruction == "do()" {
            skipping = false;
            continue;
        }

        if skipping {
            continue;
        }

        let nums: Vec<u32> = instruction
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        muls.push(Mul {
            x: nums[0],
            y: nums[1],
        });
    }

    muls
}

fn parse_input(input: &str) -> Vec<Mul> {
    let mut muls = vec![];

    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    for (_, [pair]) in re.captures_iter(input).map(|c| c.extract()) {
        let nums: Vec<u32> = pair
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        muls.push(Mul {
            x: nums[0],
            y: nums[1],
        });
    }

    muls
}

impl Problem for Day3 {
    fn part1(&self, input: &str) -> String {
        let muls = parse_input(input);
        let sum: u32 = muls.iter().map(|mul| mul.x * mul.y).sum();
        format!("{}", sum)
    }

    fn part2(&self, input: &str) -> String {
        let muls = parse_input_dos_donts(input);
        let sum: u32 = muls.iter().map(|mul| mul.x * mul.y).sum();
        format!("{}", sum)
    }
}

fn main() {
    run_problem(Day3 {}, "day3");
}
