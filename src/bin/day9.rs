use aoc24::{run_problem, Problem};

struct Day9 {}

#[derive(Debug, Copy, Clone)]
struct Block {
    file_id: u32,
}

fn parse_input(input: &str) -> Vec<Option<Block>> {
    let mut blocks = vec![];
    let mut file_id = 0;
    for (i, digit) in input.trim_end().chars().enumerate() {
        let num = digit.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..num {
                blocks.push(Some(Block { file_id }));
            }
            file_id += 1;
        } else {
            for _ in 0..num {
                blocks.push(None);
            }
        }
    }
    blocks
}

fn first_empty_block(disk: &[Option<Block>]) -> Option<usize> {
    disk.iter().position(|b| b.is_none())
}

fn are_blocks_contiguous(disk: &[Option<Block>]) -> bool {
    let block_count = disk.iter().filter(|block| block.is_some()).count();
    for i in 0..disk.len() {
        if disk[i].is_none() && i < block_count {
            return false;
        }
    }
    true
}

fn defrag_disk(disk: &mut [Option<Block>]) {
    while !are_blocks_contiguous(disk) {
        let next_empty_index = first_empty_block(disk).unwrap();
        let last_block_index = disk
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| b.is_some())
            .unwrap()
            .0;
        disk[next_empty_index] = disk[last_block_index];
        disk[last_block_index] = None;
    }
}

fn checksum(disk: &[Option<Block>]) -> u32 {
    disk.iter()
        .filter(|b| b.is_some())
        .enumerate()
        .map(|(pos, block)| pos as u32 * block.unwrap().file_id)
        .sum()
}

impl Problem for Day9 {
    fn part1(&self, input: &str) -> String {
        let mut disk = parse_input(input);
        defrag_disk(&mut disk);
        format!("defragged checksum is {}", checksum(&disk))
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

fn main() {
    run_problem(Day9 {}, "day9");
}
