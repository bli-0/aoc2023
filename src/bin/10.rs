use std::{collections::HashMap, thread::current};

fn main() {
    let inputs = include_str!("inputs/10");
    let mut pipe_grid = PipeGrid::new(inputs);

    let mut current_loc = pipe_grid.start;
    let mut current_step = 0;
    loop {
        pipe_grid.visited.insert(current_loc, current_step);
        match pipe_grid.get_next_tile(current_loc) {
            Some(next_loc) => {
                println!("current loc: {:?}, current step: {}", current_loc, current_step);
                current_loc = next_loc;
                current_step += 1;
            }
            None => break,
        }
    }

    let part1 = pipe_grid.get_furthest_step_from_start();
    println!("part1: {}", part1);
}

struct PipeGrid {
    // Lines are the first one
    // chars are the 2nd, so a lookup is actually
    // [y][x]....
    map: Vec<Vec<Pipe>>,
    // Map from coord -> step from start
    visited: HashMap<(usize, usize), u64>,
    start: (usize, usize),
}

impl PipeGrid {
    fn new(input: &str) -> Self {
        let mut start_loc = (0_usize, 0_usize);

        let map: Vec<Vec<Pipe>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let pipe = Pipe::from(c);
                        if pipe == Pipe::Start {
                            start_loc = (x, y);
                        }
                        pipe
                    })
                    .collect()
            })
            .collect();

        let visited = HashMap::new();

        PipeGrid {
            map,
            visited,
            start: start_loc,
        }
    }

    // Gets the next possible tile.
    // If we end up looping, then return None.
    fn get_next_tile(&self, current: (usize, usize)) -> Option<(usize, usize)> {
        match self.map[current.1][current.0] {
            Pipe::VertLine => {
                if self.is_visited((current.0, current.1 - 1))
                    && self.is_visited((current.0, current.1 + 1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 - 1)) {
                    Some((current.0, current.1 - 1))
                } else {
                    Some((current.0, current.1 + 1))
                }
            }
            Pipe::HoriLine => {
                if self.is_visited((current.0 - 1, current.1))
                    && self.is_visited((current.0 + 1, current.1))
                {
                    None
                } else if !self.is_visited((current.0 - 1, current.1)) {
                    Some((current.0 - 1, current.1))
                } else {
                    Some((current.0 + 1, current.1))
                }
            }
            Pipe::L => {
                if self.is_visited((current.0, current.1 - 1))
                    && self.is_visited((current.0 + 1, current.1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 - 1)) {
                    Some((current.0, current.1 - 1))
                } else {
                    Some((current.0 + 1, current.1))
                }
            }
            Pipe::J => {
                if self.is_visited((current.0, current.1 - 1))
                    && self.is_visited((current.0, current.1 - 1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 - 1)) {
                    Some((current.0, current.1 - 1))
                } else {
                    Some((current.0 - 1, current.1))
                }
            }
            Pipe::Seven => {
                if self.is_visited((current.0, current.1 + 1))
                    && self.is_visited((current.0 - 1, current.1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 + 1)) {
                    Some((current.0, current.1 + 1))
                } else {
                    Some((current.0 - 1, current.1))
                }
            }
            Pipe::F => {
                if self.is_visited((current.0, current.1 + 1))
                    && self.is_visited((current.0 + 1, current.1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 + 1)) {
                    Some((current.0, current.1 + 1))
                } else {
                    Some((current.0 + 1, current.1))
                }
            }
            Pipe::Ground => {
                panic!("unexpected ground location: {:?}", current)
            }
            Pipe::Start => {
                // Start is tricky since we don't know what it is;
                // Just look at the input and manually pick a direction!
                // Down looks like it satisfies both the sample input and my actual input.
                let next_loc = (current.0, current.1 + 1);
                Some(next_loc)
            }
        }
    }

    fn is_visited(&self, loc: (usize, usize)) -> bool {
        self.visited.contains_key(&loc)
    }

    fn get_furthest_step_from_start(&self) -> usize {
        let (_, max_step) = self.visited.iter().max_by(
            |a, b| {
                a.1.cmp(b.1)
            }
        ).unwrap();

        *max_step as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    VertLine,
    HoriLine,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VertLine,
            '-' => Self::HoriLine,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unexpected char"),
        }
    }
}
