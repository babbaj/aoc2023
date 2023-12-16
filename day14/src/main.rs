use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::ops::{Index, IndexMut};
use std::slice;
use blake2::{Blake2s256, Digest};
use blake2::digest::{FixedOutput, Output};
use crate::Square::*;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
enum Square {
    EMPTY,
    ROUND,
    CUBE,
}

struct Board(Vec<Vec<Square>>);

fn parse_board(input: &[&str]) -> Vec<Vec<Square>> {
    input
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => EMPTY,
                    '#' => CUBE,
                    'O' => ROUND,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn move_rocks<I: IndexMut<usize, Output = Square>>(line: &mut I, start: isize, end: isize, dx: isize) {
    let mut i = start;
    let mut j = start + dx;
    while j != end {
        if line[j as usize] == CUBE {
            return simulate_line(line, j, end, dx);
        }
        if line[j as usize] == ROUND {
            line[i as usize] = ROUND;
            line[j as usize] = EMPTY;
            i += dx;
        }
        j += dx;
    }
}

fn simulate_line<I: IndexMut<usize, Output = Square>>(line: &mut I, start: isize, end: isize, dx: isize) {
    let mut i= start;
    while i != end {
        if line[i as usize] == EMPTY {
            return move_rocks(line, i, end, dx);
        }
        i += dx;
    }
}

struct ColumnMut<'a> {
    i: usize,
    grid: &'a mut Vec<Vec<Square>>
}
impl Index<usize> for ColumnMut<'_> {
    type Output = Square;
    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index][self.i]
    }
}
impl IndexMut<usize> for ColumnMut<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index][self.i]
    }
}

fn load(board: &Vec<Vec<Square>>) -> usize {
    let width = board[0].len();
    let height = board.len();
    let mut out = 0;
    for row in 0..height {
        for column in 0..width {
            if board[row][column] == ROUND {
                out += height - row;
            }
        }
    }
    return out;
}

fn wash_cycle(board: &mut Vec<Vec<Square>>) {
    let width = board[0].len() as isize;
    let height = board.len() as isize;
    // north
    for i in 0..width as usize {
        simulate_line(&mut ColumnMut{i, grid: board}, 0, height, 1);
    }
    // west
    for i in 0..height as usize {
        simulate_line(&mut board[i], 0, width, 1);
    }
    // south
    for i in 0..width as usize {
        simulate_line(&mut ColumnMut{i, grid: board}, height - 1, -1, -1);
    }
    // east
    for i in 0..height as usize {
        simulate_line(&mut board[i], width - 1, -1, -1);
    }
}

type Hash = Output<Blake2s256>;

fn hash_board(board: &Vec<Vec<Square>>) -> Hash {
    let mut hasher = Blake2s256::new();
    for line in board {
        let slice = unsafe { slice::from_raw_parts(
            line.as_ptr() as *const u8,
            line.len()
        ) };
        hasher.update(slice);
    }
    return hasher.finalize_fixed();
}

fn part2(board: &mut Vec<Vec<Square>>) -> usize {
    let mut unique = HashMap::<Hash, (usize, usize)>::new();
    let mut history = Vec::<(Hash, usize)>::new();
    let mut index = 0;
    loop {
        wash_cycle(board);
        let hash = hash_board(board);
        let load = load(board);

        match unique.entry(hash) {
            Entry::Occupied(entry) => {
                println!("found cycle after {} iterations", index + 1);
                let first_index = entry.get().0;
                return history[(1_000_000_000 - history.len() - 1) % (history.len() - first_index) + first_index].1
            }
            Entry::Vacant(entry) => {
                entry.insert((index, load));
            }
        }
        history.push((hash, load));
        index += 1;
    }
}

fn main() {
    let input = include_str!("../input")
        .trim_end()
        .lines()
        .collect::<Vec<_>>();
    // left is north
    //let transposed = transpose(&input);
    let mut board = parse_board(&input);
    let height = board.len();
    /*for i in 0..board[0].len() {
        simulate_line(&mut ColumnMut{i, grid: &mut board}, 0, height as isize, 1);
    }
    println!("{}", load(&board));*/
    println!("part 2 = {}", part2(&mut board));
}
