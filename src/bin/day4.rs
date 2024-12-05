use std::collections::HashSet;

use aoc24::{parse_char_grid, run_problem, Problem};

struct Day4 {}

fn char_at_position((x, y): (i32, i32), grid: &[Vec<char>]) -> char {
    let (x, y) = (x as usize, y as usize);
    if x >= grid[0].len() || y >= grid.len() {
        return ' ';
    }

    grid[y][x]
}

fn count_word(grid: Vec<Vec<char>>, word: &str, gradients: &[(i32, i32)]) -> usize {
    let chars: Vec<char> = word.chars().collect();
    let mut found_words = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for (gradient_x, gradient_y) in gradients {
                let mut position = (x as i32, y as i32);
                let mut positions: Vec<(i32, i32)> = vec![position];

                let mut is_word = true;
                for next_char in chars.iter() {
                    if char_at_position(position, &grid) != *next_char {
                        is_word = false;
                        break;
                    }
                    position = (position.0 + gradient_x, position.1 + gradient_y);
                    positions.push(position);
                }

                if is_word {
                    found_words.insert(positions);
                }
            }
        }
    }

    found_words.len()
}

fn count_pattern(grid: Vec<Vec<char>>, patterns: Vec<Vec<Vec<char>>>) -> usize {
    let mut found_patterns = HashSet::new();

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            for pattern in patterns.iter() {
                let mut positions = vec![];

                let mut is_pattern = true;
                for pattern_y in 0..pattern.len() as i32 {
                    for pattern_x in 0..pattern[0].len() as i32 {
                        let pattern_char = char_at_position((pattern_x, pattern_y), pattern);
                        let actual_char = char_at_position((x + pattern_x, y + pattern_y), &grid);
                        if pattern_char != '*' && pattern_char != actual_char {
                            is_pattern = false;
                            break;
                        }
                        positions.push((x, y));
                    }
                }

                if is_pattern {
                    found_patterns.insert(positions);
                }
            }
        }
    }

    found_patterns.len()
}

impl Problem for Day4 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_char_grid(input);
        let count = count_word(
            grid,
            "XMAS",
            &[
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
        );
        format!("XMAS count is {}", count)
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse_char_grid(input);
        let count = count_pattern(
            grid,
            vec![
                vec![
                    vec!['M', '*', 'S'],
                    vec!['*', 'A', '*'],
                    vec!['M', '*', 'S'],
                ],
                vec![
                    vec!['S', '*', 'S'],
                    vec!['*', 'A', '*'],
                    vec!['M', '*', 'M'],
                ],
                vec![
                    vec!['S', '*', 'M'],
                    vec!['*', 'A', '*'],
                    vec!['S', '*', 'M'],
                ],
                vec![
                    vec!['M', '*', 'M'],
                    vec!['*', 'A', '*'],
                    vec!['S', '*', 'S'],
                ],
            ],
        );
        format!("X-MAS count is {}", count)
    }
}

fn main() {
    run_problem(Day4 {}, "day4");
}

#[cfg(test)]
mod tests {
    use aoc24::Problem;

    use crate::Day4;

    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part_1() {
        let day_4 = Day4 {};
        assert_eq!("XMAS count is 18", day_4.part1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        let day_4 = Day4 {};
        assert_eq!("X-MAS count is 9", day_4.part2(TEST_INPUT));
    }
}
