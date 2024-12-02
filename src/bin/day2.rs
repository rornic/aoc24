use aoc24::{run_problem, Problem};

struct Day2 {}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|level| level.parse::<u32>().unwrap())
            .collect();
        reports.push(levels);
    }
    reports
}

fn is_report_safe(report: &[u32]) -> bool {
    let differences: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| *b as i32 - *a as i32)
        .collect();
    differences
        .iter()
        .all(|diff| diff.abs() >= 1 && diff.abs() <= 3)
        && (differences.iter().all(|diff| diff.is_negative())
            || differences.iter().all(|diff| diff.is_positive()))
}

fn report_permutations(report: Vec<u32>) -> Vec<Vec<u32>> {
    let mut permutations = vec![];
    for i in 0..report.len() {
        let mut permutation = report.clone();
        permutation.remove(i);
        permutations.push(permutation);
    }
    permutations
}

impl Problem for Day2 {
    fn part1(&self, input: &str) -> String {
        let reports = parse_input(input);
        let safe = reports
            .into_iter()
            .filter(|report| is_report_safe(report))
            .count();
        format!("{}", safe)
    }

    fn part2(&self, input: &str) -> String {
        let reports = parse_input(input);
        let safe = reports
            .into_iter()
            .map(report_permutations)
            .filter(|reports| reports.iter().any(|report| is_report_safe(report)))
            .count();
        format!("{}", safe)
    }
}

fn main() {
    run_problem(Day2 {}, "day2");
}
