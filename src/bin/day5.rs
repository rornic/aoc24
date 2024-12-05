use std::{cmp::Ordering, collections::HashMap};

use aoc24::{run_problem, Problem};

struct Day5 {}

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let mut ordering_pairs = vec![];
    let mut updates = vec![];

    for line in input.lines() {
        if line.contains("|") {
            let numbers: Vec<u32> = line
                .split("|")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            ordering_pairs.push(Rule {
                before: numbers[0],
                after: numbers[1],
            });
        } else if line.contains(",") {
            let update = line
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            updates.push(update);
        }
    }

    (ordering_pairs, updates)
}

fn build_ordering_map(update: &[u32]) -> HashMap<u32, usize> {
    let mut ordering = HashMap::new();
    for (i, num) in update.iter().enumerate() {
        ordering.insert(*num, i);
    }
    ordering
}

fn is_update_valid(rules: &[Rule], update: &[u32]) -> bool {
    let ordering = build_ordering_map(update);
    rules
        .iter()
        .filter(|rule| is_rule_relevant(rule, &ordering))
        .all(|rule| is_rule_satisfied(rule, &ordering))
}

fn is_rule_satisfied(rule: &Rule, ordering: &HashMap<u32, usize>) -> bool {
    ordering.get(&rule.before).unwrap_or(&0) < ordering.get(&rule.after).unwrap_or(&1)
}

fn is_rule_relevant(rule: &Rule, ordering: &HashMap<u32, usize>) -> bool {
    ordering.contains_key(&rule.before) && ordering.contains_key(&rule.after)
}

fn sum_valid_updates(rules: &[Rule], updates: Vec<Vec<u32>>) -> u32 {
    updates
        .into_iter()
        .filter(|update| is_update_valid(rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn compare_with_rules(rules: &[Rule], n1: u32, n2: u32) -> Ordering {
    let rule = rules.iter().find(|rule| {
        (rule.before == n1 && rule.after == n2) || (rule.before == n2 && rule.after == n1)
    });
    if let Some(rule) = rule {
        if n1 == rule.before {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

impl Problem for Day5 {
    fn part1(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);
        let sum = sum_valid_updates(&rules, updates);
        format!("sum of middle numbers in valid updates is {}", sum)
    }

    fn part2(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);
        let mut invalid_updates: Vec<Vec<u32>> = updates
            .into_iter()
            .filter(|update| !is_update_valid(&rules, update))
            .collect();
        invalid_updates
            .iter_mut()
            .for_each(|update| update.sort_by(|n1, n2| compare_with_rules(&rules, *n1, *n2)));
        let sum = sum_valid_updates(&rules, invalid_updates);
        format!("sum of middle numbers in sorted updates is {}", sum)
    }
}

fn main() {
    run_problem(Day5 {}, "day5");
}

#[cfg(test)]
mod tests {
    use aoc24::Problem;

    use crate::Day5;

    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part_1() {
        let day = Day5 {};
        assert_eq!(
            "sum of middle numbers in valid updates is 143",
            day.part1(TEST_INPUT)
        );
    }

    #[test]
    fn test_part_2() {
        let day = Day5 {};
        assert_eq!(
            "sum of middle numbers in sorted updates is 123",
            day.part2(TEST_INPUT)
        );
    }
}
