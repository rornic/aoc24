use std::collections::HashSet;

use aoc24::{run_problem, Problem};

struct Day9 {}

#[derive(Debug, Copy, Clone)]
struct Block {
    file_id: u64,
}

fn parse_input(input: &str) -> Vec<Option<Block>> {
    let mut blocks = vec![];
    let mut file_id = 0;
    let mut is_block = true;
    for digit in input.trim_end().chars() {
        let num = digit.to_digit(10).unwrap();
        if is_block {
            for _ in 0..num {
                blocks.push(Some(Block { file_id }));
            }
            file_id += 1;
        } else {
            for _ in 0..num {
                blocks.push(None);
            }
        }
        is_block = !is_block;
    }
    blocks
}

fn defrag_disk(disk: &mut [Option<Block>]) {
    let mut first_empty_index = first_empty_block(disk, 1, disk.len()).unwrap();

    for i in (0..disk.len()).rev() {
        if disk[i].is_none() {
            continue;
        }

        let last_non_empty_index = disk
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| b.is_some())
            .unwrap()
            .0;
        disk[first_empty_index] = disk[last_non_empty_index];
        disk[last_non_empty_index] = None;

        while disk[first_empty_index].is_some() {
            first_empty_index += 1;
        }
        if i - first_empty_index <= 1 {
            break;
        }
    }
}

fn first_empty_block(disk: &[Option<Block>], size: usize, max_idx: usize) -> Option<usize> {
    let mut is_empty = false;
    let mut empty_start: usize = 0;
    for i in 0..=max_idx {
        if disk[i].is_some() && is_empty {
            let length = i - empty_start;
            if length >= size {
                return Some(empty_start);
            }
            is_empty = false;
        }

        if !is_empty && disk[i].is_none() {
            is_empty = true;
            empty_start = i;
        }
    }
    None
}

fn last_file_block(disk: &[Option<Block>], seen_files: &HashSet<u64>) -> (usize, usize, u64) {
    let mut is_tracking_file = false;
    let mut file_id = 0;
    let mut file_end: usize = 0;
    for i in (0..disk.len()).rev() {
        if let Some(b) = disk[i] {
            if !seen_files.contains(&b.file_id) && !is_tracking_file {
                is_tracking_file = true;
                file_id = b.file_id;
                file_end = i;
            }
        }

        if is_tracking_file && (disk[i].is_none() || disk[i].unwrap().file_id != file_id) {
            return (i + 1, file_end - i, file_id);
        }
    }

    (0, 0, 0)
}

fn defrag_disk_whole_files(disk: &mut [Option<Block>]) {
    let mut seen_files = HashSet::new();

    while !seen_files.contains(&0) {
        let (file_start, file_length, file_id) = last_file_block(disk, &seen_files);
        if let Some(empty_start) = first_empty_block(disk, file_length, file_start) {
            for i in file_start..(file_start + file_length) {
                disk[empty_start + (i - file_start)] = disk[i];
                disk[i] = None;
            }
        }
        seen_files.insert(file_id);
    }
}

fn checksum(disk: &[Option<Block>]) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(pos, block)| {
            if let Some(b) = block {
                pos as u64 * b.file_id
            } else {
                0
            }
        })
        .sum()
}

impl Problem for Day9 {
    fn part1(&self, input: &str) -> String {
        let mut disk = parse_input(input);
        defrag_disk(&mut disk);
        format!("defragged checksum is {}", checksum(&disk))
    }

    fn part2(&self, input: &str) -> String {
        let mut disk = parse_input(input);
        println!("{:?}", last_file_block(&disk, &HashSet::new()));
        defrag_disk_whole_files(&mut disk);
        format!("defragged checksum for whole files is {}", checksum(&disk))
    }
}

fn main() {
    run_problem(Day9 {}, "day9");
}
