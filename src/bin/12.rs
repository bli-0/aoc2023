use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/12");
    let records: Vec<Record> = inputs.lines().map(Record::from_line).collect();
    let possible_arrangements: Vec<u64> = records
        .iter()
        .map(|record| record.possible_arrangements())
        .collect();

    let part1: u64 = possible_arrangements.iter().sum();
    println!("part1: {}", part1);

    let records2: Vec<Record> = records
        .iter()
        .map(|record| {
            let mut new_row: Vec<char> = vec![];
            let mut new_springs = vec![];

            for i in 0..5 {
                new_row.extend(record.row.iter());
                if i != 4 {
                    new_row.push('?');
                }
                new_springs.extend(record.springs.iter());
            }

            Record {
                row: new_row,
                springs: new_springs,
            }
        })
        .collect();

    let mut possible_arrangements2 = vec![];

    for (i, record) in records2.iter().enumerate() {
        possible_arrangements2.push(record.possible_arrangements());
        println!("finished record: {}", i);
    }

    // let possible_arrangements2: Vec<u64> = records2
    //     .iter()
    //     .enumerate()
    //     .map(|(i, record)| {
    //         let new_record = record.possible_arrangements();
    //         println!("finished record: {}", i);
    //         new_record
    //     })
    //     .collect();
    let part2: u64 = possible_arrangements2.iter().sum();
    println!("part2: {}", part2);
}

#[derive(Debug)]
struct Record {
    row: Vec<char>,
    springs: Vec<u64>,
}

impl Record {
    fn from_line(line: &str) -> Self {
        let (row_str, springs_str) = line.split_once(' ').unwrap();

        Record {
            row: row_str.chars().collect_vec(),
            springs: springs_str.split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn possible_arrangements(&self) -> u64 {
        Record::possible_arrangements_inner(&self.row, &self.springs, false, false)
    }

    fn possible_arrangements_inner(
        remainder: &[char],
        to_resolve: &[u64],
        midway: bool,
        needs_gap: bool,
    ) -> u64 {
        // The moment we hit a ?, we recurse twice to cover the two options.
        // Eliminate impossible options.
        let mut new_to_resolve = vec![0; to_resolve.len()];
        new_to_resolve.clone_from_slice(to_resolve);

        match remainder.first() {
            Some(c) => {
                match c {
                    '?' => {
                        let mut possible_arrangements = 0;
                        let mut remainder1 = vec![' '; remainder.len()];
                        let mut remainder2 = vec![' '; remainder.len()];
                        remainder1.clone_from_slice(remainder);
                        remainder2.clone_from_slice(remainder);

                        remainder1[0] = '#';
                        remainder2[0] = '.';
                        possible_arrangements += Record::possible_arrangements_inner(
                            &remainder1,
                            to_resolve,
                            midway,
                            needs_gap,
                        );
                        possible_arrangements += Record::possible_arrangements_inner(
                            &remainder2,
                            to_resolve,
                            midway,
                            needs_gap,
                        );

                        possible_arrangements
                    }
                    '.' => {
                        // If we encounter a dot and we are midway in resolving a group, then this is an impossiblitiy.
                        if midway {
                            0
                        } else {
                            let new_remainder: &[char] = &remainder[1..];
                            Record::possible_arrangements_inner(
                                new_remainder,
                                &new_to_resolve,
                                false,
                                false,
                            )
                        }
                    }
                    '#' => {
                        if needs_gap || new_to_resolve.is_empty() {
                            return 0;
                        }
                        let new_remainder: &[char] = &remainder[1..];
                        let new_midway: bool;
                        let new_needs_gap: bool;

                        // We've gotten to the end of a block.
                        if (new_to_resolve[0] - 1) == 0 {
                            new_to_resolve = new_to_resolve[1..].to_vec();
                            new_midway = false;
                            new_needs_gap = true;
                        } else {
                            new_to_resolve[0] -= 1;
                            new_midway = true;
                            new_needs_gap = false;
                        };

                        Record::possible_arrangements_inner(
                            new_remainder,
                            &new_to_resolve,
                            new_midway,
                            new_needs_gap,
                        )
                    }
                    _ => {
                        panic!("unexpected char")
                    }
                }
            }
            None => {
                // There are no chars left, but we expect some more springs.
                if to_resolve.is_empty() {
                    0
                } else {
                    1
                }
            }
        }
    }
}
