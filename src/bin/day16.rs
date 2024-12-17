use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc24::{parse_char_grid, run_problem, Problem};

struct Day16 {}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse_char_grid(input)
}

fn char_at((x, y): (i32, i32), grid: &[Vec<char>]) -> char {
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return '#';
    }

    let (x, y) = (x as usize, y as usize);
    grid[y][x]
}

fn position(c: char, grid: &[Vec<char>]) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == c {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("no char found")
}

#[derive(PartialEq, Eq)]
struct HeapItem {
    path: Vec<(i32, i32)>,
    dir: (i32, i32),
    score: u32,
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn shortest_paths(grid: &[Vec<char>]) -> Vec<(Vec<(i32, i32)>, u32)> {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let start = position('S', grid);
    heap.push(HeapItem {
        path: vec![start],
        dir: (1, 0),
        score: 0,
    });
    seen.insert((start.0, start.1, (1, 0)));

    let mut paths = vec![];

    while let Some(heap_item) = heap.pop() {
        let (path, direction, score) = (heap_item.path, heap_item.dir, heap_item.score);
        let (pos_x, pos_y) = path.last().unwrap();

        if char_at((*pos_x, *pos_y), grid) == 'E' {
            if let Some((_, s)) = paths.first() {
                if *s == score {
                    paths.push((path.clone(), score))
                } else {
                    return paths;
                }
            } else {
                paths.push((path.clone(), score))
            }
            continue;
        }

        seen.insert((*pos_x, *pos_y, direction));

        let new_positions: Vec<(i32, i32)> = [(0, 1), (1, 0), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(dir_x, dir_y)| (*pos_x + dir_x, *pos_y + dir_y))
            .filter(|pos| {
                char_at((pos.0, pos.1), grid) == '.' || char_at((pos.0, pos.1), grid) == 'E'
            })
            .filter(|pos| !seen.contains(&(pos.0, pos.1, direction)))
            .collect();

        for new_pos in new_positions {
            let mut new_path = path.clone();
            new_path.push(new_pos);

            let new_dir = (new_pos.0 - pos_x, new_pos.1 - pos_y);
            let (diff_x, diff_y) = (
                new_dir.0.abs_diff(direction.0),
                new_dir.1.abs_diff(direction.1),
            );
            let turns = diff_x.max(diff_y);

            let new_score = score + 1 + (1000 * turns);
            heap.push(HeapItem {
                path: new_path,
                dir: new_dir,
                score: new_score,
            });
        }
    }

    panic!("no path found")
}

impl Problem for Day16 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let paths = shortest_paths(&grid);
        let (path, score) = paths[0].clone();
        format!("shortest path has score {} and {} steps", score, path.len())
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse_input(input);
        let paths = shortest_paths(&grid);
        let unique_positions: HashSet<(i32, i32)> =
            paths.into_iter().flat_map(|(path, _)| path).collect();
        format!(
            "all shortest path touch {} unique positions",
            unique_positions.len()
        )
    }
}

fn main() {
    run_problem(Day16 {}, "day16");
}
