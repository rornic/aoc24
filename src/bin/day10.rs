use std::collections::{HashMap, HashSet, VecDeque};

use aoc24::{run_problem, Problem};

struct Day10 {}

type Trail = Vec<(i32, i32)>;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut rows = vec![];
    for line in input.lines() {
        let row = line
            .chars()
            .map(|num| num.to_digit(10).unwrap() as i32)
            .collect();
        rows.push(row);
    }
    rows
}

fn find_trails(topography: Vec<Vec<i32>>) -> Vec<Trail> {
    let mut trails = vec![];
    for y in 0..topography.len() {
        for x in 0..topography[0].len() {
            if topography[y][x] == 0 {
                trails.extend(find_trails_from_start(&topography, (x as i32, y as i32)))
            }
        }
    }
    trails
}

fn find_trails_from_start(topography: &[Vec<i32>], start: (i32, i32)) -> Vec<Trail> {
    let mut trails = vec![];
    let mut stack = VecDeque::new();
    stack.push_back(vec![start]);

    while let Some(trail) = stack.pop_back() {
        let (last_x, last_y) = trail.last().unwrap();
        if topography[*last_y as usize][*last_x as usize] == 9 {
            trails.push(trail);
        } else {
            let adjacent = adjacent(topography, (*last_x, *last_y));
            for adjacent_step in adjacent {
                let mut new_trail = trail.clone();
                new_trail.push(adjacent_step);
                stack.push_back(new_trail);
            }
        }
    }

    trails
}

fn adjacent(topography: &[Vec<i32>], (pos_x, pos_y): (i32, i32)) -> Vec<(i32, i32)> {
    let value = topography[pos_y as usize][pos_x as usize];
    vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
        .into_iter()
        .map(|(dir_x, dir_y)| (pos_x + dir_x, pos_y + dir_y))
        .filter(|(pos_x, pos_y)| {
            *pos_x >= 0
                && *pos_y >= 0
                && *pos_y < topography.len() as i32
                && *pos_x < topography[0].len() as i32
                && topography[*pos_y as usize][*pos_x as usize] - value == 1
        })
        .collect()
}

fn calculate_trail_scores(trails: Vec<Trail>, unique_ends: bool) -> HashMap<(i32, i32), u32> {
    let mut trailhead_score_map = HashMap::new();

    let trailheads: HashSet<(i32, i32)> = trails.iter().map(|trail| trail[0]).collect();
    for trailhead in trailheads {
        if unique_ends {
            let trail_ends: HashSet<(i32, i32)> = trails
                .iter()
                .filter(|trail| trail[0] == trailhead)
                .map(|trail| trail[trail.len() - 1])
                .collect();
            trailhead_score_map.insert(trailhead, trail_ends.len() as u32);
        } else {
            let unique_trails: HashSet<Trail> = trails
                .iter()
                .filter(|trail| trail[0] == trailhead)
                .cloned()
                .collect();
            trailhead_score_map.insert(trailhead, unique_trails.len() as u32);
        };
    }
    trailhead_score_map
}

impl Problem for Day10 {
    fn part1(&self, input: &str) -> String {
        let topography = parse_input(input);
        let trails = find_trails(topography);
        let scores = calculate_trail_scores(trails, true);
        format!(
            "sum of trailhead scores is {}",
            scores.values().sum::<u32>()
        )
    }

    fn part2(&self, input: &str) -> String {
        let topography = parse_input(input);
        let trails = find_trails(topography);
        let scores = calculate_trail_scores(trails, false);
        format!(
            "sum of trailhead ratings is {}",
            scores.values().sum::<u32>()
        )
    }
}

fn main() {
    run_problem(Day10 {}, "day10");
}
