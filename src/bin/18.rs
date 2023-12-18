use std::{
    collections::{HashSet, VecDeque},
};

use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/18");
    let instructions: Vec<Instruction> = inputs.lines().map(Instruction::from_str).collect();

    let mut current_point: (i64, i64) = (500, 500);
    let mut grid = vec![vec![Space::new(0); 1000]; 1000];
    for inst in instructions.iter() {
        match inst.direction {
            Direction::U => {
                let new_point = (current_point.0 - inst.amount as i64, current_point.1);
                for i in new_point.0..current_point.0 {
                    grid[i as usize][new_point.1 as usize].color = inst.hex;
                    grid[i as usize][new_point.1 as usize].is_dug = true;
                }
                current_point = new_point;
            }
            Direction::D => {
                let new_point = (current_point.0 + inst.amount as i64, current_point.1);
                for i in current_point.0 + 1..=new_point.0 {
                    grid[i as usize][new_point.1 as usize].color = inst.hex;
                    grid[i as usize][new_point.1 as usize].is_dug = true;
                }
                current_point = new_point;
            }
            Direction::L => {
                let new_point = (current_point.0, current_point.1 - inst.amount as i64);
                for j in new_point.1..current_point.1 {
                    grid[new_point.0 as usize][j as usize].color = inst.hex;
                    grid[new_point.0 as usize][j as usize].is_dug = true;
                }
                current_point = new_point;
            }
            Direction::R => {
                let new_point = (current_point.0, current_point.1 + inst.amount as i64);
                for j in current_point.1 + 1..=new_point.1 {
                    grid[new_point.0 as usize][j as usize].color = inst.hex;
                    grid[new_point.0 as usize][j as usize].is_dug = true;
                }
                current_point = new_point;
            }
        }
    }

    mark_inner(&mut grid);
    print_grid(&grid);
    let part1 = count_inner(&grid);
    println!("part1: {}", part1);

    // Part 2 - > completely fucks the model for p1.
    // After some googling: https://en.wikipedia.org/wiki/Pick%27s_theorem + https://en.wikipedia.org/wiki/Shoelace_formula
    let mut current_vertex = (0,0);
    let mut corners = vec![];
    for inst in instructions.iter() {
        match inst.direction {
            Direction::U => current_vertex.0 += (inst.amount) as i64,
            Direction::D => current_vertex.0 -= (inst.amount) as i64,
            Direction::L => current_vertex.1 -= (inst.amount) as i64,
            Direction::R => current_vertex.1 += (inst.amount) as i64,
        }
        corners.push(current_vertex);
    }

    let part2 = area_theorem(&corners, &instructions);
    println!("part2: {}", part2);

}

#[allow(unused)]
fn print_grid(grid: &[Vec<Space>]) {
    println!();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_dug {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

// This is all used in part 1 that got binned in part2.
// fn mark_inner(grid: &mut [Vec<Space>]) {
//     let mut outer = HashSet::<(usize, usize)>::new();

//     let mut queue = VecDeque::<(usize, usize)>::new();
//     queue.push_front((0, 0));

//     while let Some(next) = queue.pop_front() {
//         if grid[next.0][next.1].is_dug || outer.contains(&next) {
//             continue;
//         }

//         outer.insert(next);
//         if next.0 > 0 {
//             queue.push_back((next.0 - 1, next.1));
//         }
//         if next.0 < grid.len() - 1 {
//             queue.push_back((next.0 + 1, next.1));
//         }
//         if next.1 > 0 {
//             queue.push_back((next.0, next.1 - 1));
//         }
//         if next.1 < grid[0].len() - 1 {
//             queue.push_back((next.0, next.1 + 1));
//         }
//     }

//     for i in 0..grid.len() {
//         for j in 0..grid[0].len() {
//             if !outer.contains(&(i,j)) {
//                 grid[i][j].is_dug = true;
//             }
//         }
//     }
// }

// fn count_inner(grid: &[Vec<Space>]) -> u64 {
//     let mut sum = 0;
//     for i in 0..grid.len() {
//         for j in 0..grid[0].len() {
//             if grid[i][j].is_dug {
//                 sum += 1;
//             }
//         }
//     }
//     sum
// }

// Picks theorem is: 
// Area = Sum of Interior + Sum of Perimeter/2 + 1;
fn area_theorem(corners: &[(i64, i64)], instructions: &[Instruction]) -> u64 {
    // Shoelace
    let mut sum_of_determinants: i64 = 0;

    for i in 0..corners.len() {
        if i != corners.len() - 1 {
            sum_of_determinants +=
                (corners[i].0 * corners[i + 1].1) - (corners[i].1 * corners[i + 1].0);
        } else {
            sum_of_determinants += (corners[i].0 * corners[0].1) - (corners[i].1 * corners[0].0);
        }
    }

    // Make sure this area is a whole number since our coordinates are only integers.
    debug_assert!((sum_of_determinants % 2) == 0);
    let area: u64 = (sum_of_determinants.abs() / 2).try_into().unwrap();

    let mut perimeter_length = 0;
    for i in instructions {
        perimeter_length += i.amount;
    }

    area + perimeter_length/2 + 1
}

struct Instruction {
    direction: Direction,
    amount: u64,
    hex: u64,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let (direction_str, amount_str, hex_str) = s.splitn(3, ' ').collect_tuple().unwrap();

        Self {
            direction: Direction::from_str(direction_str),
            amount: amount_str.parse().unwrap(),
            hex: u64::from_str_radix(&hex_str[2..hex_str.len() - 1], 16).unwrap(),
        }
    }
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::U,
            "D" => Self::D,
            "L" => Self::L,
            "R" => Self::R,
            _ => panic!("unexpected str"),
        }
    }
}

#[derive(Copy, Clone)]
struct Space {
    color: u64,
    is_dug: bool,
}

impl Space {
    fn new(color: u64) -> Self {
        Self {
            color,
            is_dug: false,
        }
    }
}
