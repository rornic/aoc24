use std::collections::VecDeque;

use aoc24::{run_problem, Problem};

struct Day7 {}

#[derive(Copy, Clone, Debug)]
enum Token {
    Operator(char),
    Operand(u64),
}

impl Token {
    fn operator(self) -> char {
        match self {
            Token::Operator(c) => c,
            Token::Operand(_) => panic!("not an operand"),
        }
    }

    fn operand(self) -> u64 {
        match self {
            Token::Operator(_) => panic!("not an operand"),
            Token::Operand(n) => n,
        }
    }
}

fn parse_input(input: &str) -> Vec<(u64, VecDeque<Token>)> {
    let mut equations = vec![];

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let test_value = split[0].parse().unwrap();
        let tokens = split[1]
            .split(" ")
            .map(|num| Token::Operand(num.parse().unwrap()))
            .collect();
        equations.push((test_value, tokens))
    }

    equations
}

fn explode_equation(queue: &VecDeque<Token>, concat: bool) -> Vec<VecDeque<Token>> {
    let mut permutations = vec![];

    let mut permutation_stack: VecDeque<(usize, VecDeque<Token>)> = VecDeque::new();
    permutation_stack.push_back((0, queue.clone()));

    while let Some((index, next)) = permutation_stack.pop_front() {
        if index < next.len() {
            if index > 0 {
                let mut mul = next.clone();
                mul.insert(index, Token::Operator('*'));
                permutation_stack.push_back((index + 2, mul));
            }

            if concat {
                let mut concat = next.clone();
                concat.insert(index, Token::Operator('|'));
                permutation_stack.push_back((index + 2, concat));
            }

            let mut add = next.clone();
            add.insert(index, Token::Operator('+'));
            permutation_stack.push_back((index + 2, add));
        } else {
            permutations.push(next.clone());
        }
    }

    permutations
}

fn evaluate_equation(queue: &mut VecDeque<Token>) -> u64 {
    let mut accumulator = 0;

    while !queue.is_empty() {
        let operator = queue.pop_front().unwrap().operator();
        let operand = queue.pop_front().unwrap().operand();

        match operator {
            '*' => accumulator *= operand,
            '+' => accumulator += operand,
            '|' => {
                accumulator = format!("{}{}", accumulator, operand).parse().unwrap();
            }
            _ => panic!("unknown operator"),
        };
    }

    accumulator
}

fn is_equation_possible((test_value, equation): &(u64, VecDeque<Token>), concat: bool) -> bool {
    let mut permutations = explode_equation(equation, concat);
    permutations
        .iter_mut()
        .any(|permutation| evaluate_equation(permutation) == *test_value)
}

impl Problem for Day7 {
    fn part1(&self, input: &str) -> String {
        let equations = parse_input(input);
        let sum: u64 = equations
            .iter()
            .filter(|equation| is_equation_possible(equation, false))
            .map(|(test_value, _)| *test_value)
            .sum();
        format!("sum of possible equations is {}", sum)
    }

    fn part2(&self, input: &str) -> String {
        let equations = parse_input(input);
        let sum: u64 = equations
            .iter()
            .filter(|equation| is_equation_possible(equation, true))
            .map(|(test_value, _)| *test_value)
            .sum();
        format!("sum of possible equations using concat is {}", sum)
    }
}

fn main() {
    run_problem(Day7 {}, "day7");
}
