use std::collections::HashSet;

use aoc24::{parse_char_grid, run_problem, Problem};

struct Day6 {}

#[derive(PartialEq, Debug)]
enum GridSpace {
    Empty,
    Guard,
    Blocked,
}

fn object_at((x, y): (i32, i32), grid: &[Vec<char>]) -> Option<GridSpace> {
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return None;
    }

    let (x, y) = (x as usize, y as usize);
    Some(match grid[y][x] {
        '.' => GridSpace::Empty,
        '^' | '>' | 'v' | '<' => GridSpace::Guard,
        '#' => GridSpace::Blocked,
        _ => GridSpace::Empty,
    })
}

fn rotate_guard(guard: char) -> char {
    match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("invalid guard character {}", guard),
    }
}

fn guard_step(guard: char) -> (i32, i32) {
    match guard {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("invalid guard character {}", guard),
    }
}

fn initial_guard_position(grid: &[Vec<char>]) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("no guard found")
}

fn calculate_guard_path(grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut visited_positions = vec![];

    let mut guard_pos = initial_guard_position(grid);
    let mut next_pos = (guard_pos.0, guard_pos.1 - 1);
    let mut guard_symbol: char = '^';
    while let Some(space) = object_at(next_pos, grid) {
        visited_positions.push(guard_pos);

        if visited_positions.len() > 10000 {
            break; // probably looping
        }

        match space {
            GridSpace::Blocked => {
                guard_symbol = rotate_guard(guard_symbol);
            }
            GridSpace::Empty => {
                guard_pos = next_pos;
            }
            GridSpace::Guard => guard_pos = next_pos,
        };

        let (step_x, step_y) = guard_step(guard_symbol);
        next_pos = (guard_pos.0 + step_x, guard_pos.1 + step_y);
    }
    visited_positions.push(guard_pos);
    visited_positions
}

fn is_path_looping(path: &[(i32, i32)]) -> bool {
    path.len() >= 10000
}

fn obstacle_permutations(grid: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let path = calculate_guard_path(&grid);
    let path_set: HashSet<(i32, i32)> = HashSet::from_iter(path.clone());
    let new_obstacle_positions: HashSet<(i32, i32)> = path_set
        .iter()
        .flat_map(|pos| {
            vec![
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
            ]
        })
        .filter(|pos| object_at(*pos, &grid) == Some(GridSpace::Empty))
        .collect();

    let mut permutations = vec![];
    for (x, y) in new_obstacle_positions {
        let mut new_grid = grid.clone();
        new_grid[y as usize][x as usize] = '#';
        permutations.push(new_grid);
    }
    permutations
}

impl Problem for Day6 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_char_grid(input);
        let path = calculate_guard_path(&grid);
        let unique_positions: HashSet<(i32, i32)> = HashSet::from_iter(path);
        format!("guard visited {} unique positions", unique_positions.len())
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse_char_grid(input);
        let permutations = obstacle_permutations(grid);
        let looping_count = permutations
            .into_iter()
            .map(|permutation| calculate_guard_path(&permutation))
            .filter(|path| is_path_looping(path))
            .count();
        format!(
            "there are {} obstacle positions that could cause a loop",
            looping_count
        )
    }
}

fn main() {
    run_problem(Day6 {}, "day6");
}
