extern crate core;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use crate::Direction::{Down, Left, Right, Up};

fn main() {
    let movement_instructions: String = fs::read_to_string("./day09/src/input.txt").expect("Could not open input file");
    let visited: i32 = tail_visited(movement_instructions.clone(), false);
    println!("🪢 tail visited part1: {visited} positions");
    let visited2: i32 = tail_visited(movement_instructions, true);
    println!("🐍 tail visited part2: {visited2} positions");
}

fn tail_visited(movement_instructions: String, part2: bool) -> i32 {
    let instructions = parse_instructions(movement_instructions);
    let mut visited_part: HashSet<(i32, i32)> = HashSet::new();

    if part2 {
        let knots = vec![Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }, Knot { row: 0, column: 0 }];

        let mut rope = RopeMultiKnot {
            leader: Knot { row: 0, column: 0 },
            followers: knots
        };

        visited_part.insert((0, 0));

        for instruction in instructions {
            let direction = instruction.0;
            let distance = instruction.1;

            // Move one step at a time
            for _ in 0..distance {
                rope = rope.move_head(direction);
                // Update visited
                let tail = rope.followers.last().expect("Must have a tail");
                visited_part.insert((tail.row, tail.column));
            }
        }
        
    } else {
        // Starting positions
        let mut rope = Rope {
            head: Knot { row: 0, column: 0 },
            tail: Knot { row: 0, column: 0 },
        };

        visited_part.insert((rope.tail.row, rope.tail.column));

        for instruction in instructions {
            let direction = instruction.0;
            let distance = instruction.1;

            // Move one step at a time
            for _ in 0..distance {
                rope = rope.move_head(direction);
                // Update visited
                visited_part.insert((rope.tail.row, rope.tail.column));
            }
        }
    }

    visited_part.len() as i32
}

fn parse_instructions(instructions: String) -> Vec<(Direction, i32)> {
    let mut parsed_instructions = Vec::new();
    for instruction in instructions.lines() {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let direction = Direction::from_str(parts[0]).expect("Error in direction input");
        let distance = parts[1].parse::<i32>().expect("Error in direction number");
        parsed_instructions.push((direction, distance));
    }
    parsed_instructions
}

#[derive(Debug, PartialEq, Clone)]
struct Rope {
    head: Knot,
    tail: Knot,
}

impl Rope {
    fn move_head(&self, direction: Direction) -> Rope {
        let new_head = match direction {
            Up => { Knot { row: self.head.row + 1, column: self.head.column } }
            Down => { Knot { row: self.head.row - 1, column: self.head.column } }
            Right => { Knot { row: self.head.row, column: self.head.column + 1 } }
            Left => { Knot { row: self.head.row, column: self.head.column - 1 } }
        };


        // Move tail
        let x = !new_head.is_touching(self.tail);

        let new_tail = if x {
            // Head is not touching
            // Diagonal
            if (self.head.column != self.tail.column) && (self.head.row != self.tail.row) {
                let new_column = match self.head.column.cmp(&self.tail.column) {
                    Ordering::Less => { self.tail.column - 1 }
                    Ordering::Equal => { self.tail.column }
                    Ordering::Greater => { self.tail.column + 1 }
                };
                Knot { row: self.head.row, column: new_column }
            } else {
                // Same column or row
                let new_row = match self.head.row.cmp(&self.tail.row) {
                    Ordering::Less => { self.tail.row - 1 }
                    Ordering::Equal => { self.tail.row }
                    Ordering::Greater => { self.tail.row + 1 }
                };

                let new_column = match self.head.column.cmp(&self.tail.column) {
                    Ordering::Less => { self.tail.column - 1 }
                    Ordering::Equal => { self.tail.column }
                    Ordering::Greater => { self.tail.column + 1 }
                };
                Knot { row: new_row, column: new_column }
            }
        } else { self.tail };


        // Return rope
        Rope {
            head: new_head,
            tail: new_tail,
        }
    }
}

struct RopeMultiKnot {
    leader: Knot,
    followers: Vec<Knot>,
}

impl RopeMultiKnot {
    fn move_head(&self, direction: Direction) -> RopeMultiKnot {
        let new_head = match direction {
            Up => { Knot { row: self.leader.row + 1, column: self.leader.column } }
            Down => { Knot { row: self.leader.row - 1, column: self.leader.column } }
            Right => { Knot { row: self.leader.row, column: self.leader.column + 1 } }
            Left => { Knot { row: self.leader.row, column: self.leader.column - 1 } }
        };

        let mut new_followers: Vec<Knot> = Vec::new();

        // Move other knots
        let mut leader = new_head.clone();
        for follower in self.followers.iter() {
            let not_in_contact = !leader.is_touching(*follower);
            let new_follower = if not_in_contact {
                // Head is not touching
                // Diagonal
                if (leader.column != follower.column) && (leader.row != follower.row) {
                    let new_column = match leader.column.cmp(&follower.column) {
                        Ordering::Less => { follower.column - 1 }
                        Ordering::Equal => { follower.column }
                        Ordering::Greater => { follower.column + 1 }
                    };
                    Knot { row: leader.row, column: new_column }
                } else {
                    // Same column or row
                    let new_row = match leader.row.cmp(&follower.row) {
                        Ordering::Less => { follower.row - 1 }
                        Ordering::Equal => { follower.row }
                        Ordering::Greater => { follower.row + 1 }
                    };

                    let new_column = match leader.column.cmp(&follower.column) {
                        Ordering::Less => { follower.column - 1 }
                        Ordering::Equal => { follower.column }
                        Ordering::Greater => { follower.column + 1 }
                    };
                    Knot { row: new_row, column: new_column }
                }
            } else { *follower };
            leader = new_follower;
            new_followers.push(new_follower);
        }

        // Return rope
        RopeMultiKnot {
            leader: new_head,
            followers: new_followers,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Knot {
    row: i32,
    column: i32,
}

impl Knot {
    fn is_touching(&self, other: Knot) -> bool {
        let column_diff = (self.column - other.column).abs();
        let row_diff = (self.row - other.row).abs();
        !(row_diff > 1 || column_diff > 1)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const MOVEMENTS1: &str = indoc! {"R 4
                                     U 4
                                     L 3
                                     D 1
                                     R 4
                                     D 1
                                     L 5
                                     R 2"};
    const MOVEMENTS2: &str = indoc! {"R 5
                                      U 8
                                      L 8
                                      D 3
                                      R 17
                                      D 10
                                      L 25
                                      U 20"};

    #[test]
    fn test_tail_visited_part1() {
        let visited = tail_visited(MOVEMENTS1.to_string(), false);
        assert_eq!(visited, 13);
    }

    #[test]
    #[ignore]
    fn test_tail_visited_part2_simple() {
        let visited = tail_visited(MOVEMENTS1.to_string(), true);
        assert_eq!(visited, 9);
    }
    #[test]
    #[ignore]
    fn test_tail_visited_part2_complicated() {
        let visited = tail_visited(MOVEMENTS2.to_string(), true);
        assert_eq!(visited, 36);
    }
}

