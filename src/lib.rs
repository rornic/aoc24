use std::io;

pub trait Problem {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn run_problem(problem: impl Problem, input_filename: &str) {
    let input = read_input(input_filename).expect("failed to read input");

    println!("solving {}...", input_filename);
    println!("running part 1...");
    println!("{}", problem.part1(&input));

    println!("running part 2...");
    println!("{}", problem.part2(&input));
}

pub fn read_input(name: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(format!("input/{}", name))
}
