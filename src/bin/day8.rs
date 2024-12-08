use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use aoc24::{parse_char_grid, run_problem, Problem};

struct Day8 {}

fn parse_antenna_map(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antenna_map = HashMap::new();
    let grid = parse_char_grid(input);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                antenna_map
                    .entry(grid[y][x])
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    antenna_map
}

fn grid_dimensions(input: &str) -> (usize, usize) {
    let grid = parse_char_grid(input);
    (grid[0].len(), grid.len())
}

fn antenna_pairings(antennae: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    let mut pairings = vec![];
    for (i, antenna) in antennae.iter().enumerate() {
        pairings.extend(
            antennae
                .iter()
                .enumerate()
                .filter_map(|(j, a)| if j != i { Some(*a) } else { None })
                .zip(repeat(*antenna)),
        );
    }
    pairings
}

fn antinode_locations(
    pairings: Vec<((i32, i32), (i32, i32))>,
    steps: usize,
) -> HashSet<(i32, i32)> {
    let mut locations = HashSet::new();

    for ((x1, y1), (x2, y2)) in pairings {
        let (diff_x, diff_y) = ((x2 - x1), (y2 - y1));

        let negative_steps = diff_steps((x1, y1), (-diff_x, -diff_y), steps);
        let positive_steps = diff_steps((x2, y2), (diff_x, diff_y), steps);
        locations.extend(positive_steps);
        locations.extend(negative_steps)
    }

    locations
}

fn diff_steps((x, y): (i32, i32), (dir_x, dir_y): (i32, i32), steps: usize) -> Vec<(i32, i32)> {
    let mut locations = vec![];
    let (mut x, mut y) = (x, y);
    if steps > 1 {
        locations.push((x, y));
    }

    for _ in 0..steps {
        x += dir_x;
        y += dir_y;
        locations.push((x, y));
    }
    locations
}

fn all_valid_antinodes(
    antenna_map: HashMap<char, Vec<(i32, i32)>>,
    (grid_width, grid_height): (usize, usize),
    steps: usize,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    for (_, antennae) in antenna_map {
        let pairings = antenna_pairings(&antennae);
        antinodes.extend(antinode_locations(pairings, steps).iter().filter(|(x, y)| {
            *x >= 0 && *y >= 0 && (*x as usize) < grid_width && (*y as usize) < grid_height
        }));
    }
    antinodes
}

impl Problem for Day8 {
    fn part1(&self, input: &str) -> String {
        let grid_dimensions = grid_dimensions(input);
        let antenna_map = parse_antenna_map(input);
        let antinodes = all_valid_antinodes(antenna_map, grid_dimensions, 1);
        format!("there are {} unique antinodes", antinodes.len())
    }

    fn part2(&self, input: &str) -> String {
        let grid_dimensions = grid_dimensions(input);
        let antenna_map = parse_antenna_map(input);
        let antinodes = all_valid_antinodes(antenna_map, grid_dimensions, 50);
        format!("there are {} unique antinodes", antinodes.len())
    }
}

fn main() {
    run_problem(Day8 {}, "day8")
}
