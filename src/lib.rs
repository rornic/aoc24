use std::io;

pub trait Problem {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn run_problem(problem: impl Problem, input_filename: &str) {
    let input = read_input(input_filename).expect("failed to read input");

    println!("{}", problem.part1(&input));
    println!("{}", problem.part2(&input));
}

pub fn read_input(name: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(format!("input/{}", name))
}

pub fn parse_char_grid(grid: &str) -> Vec<Vec<char>> {
    let mut rows = vec![];
    for line in grid.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

pub fn parse_number_grid(grid: &str) -> Vec<Vec<i32>> {
    let mut rows = vec![];
    for line in grid.lines() {
        let row = line
            .split(" ")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        rows.push(row);
    }
    rows
}
