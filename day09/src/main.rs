extern crate core;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use crate::Direction::{Down, Left, Right, Up};

fn main() {
    let movement_instructions: String = fs::read_to_string("./day09/src/input.txt").expect("Could not open input file");
    let visited: i32 = tail_visited(movement_instructions);
    print!("🪢 tail visited: {visited} positions");
}

fn tail_visited(movement_instructions: String) -> i32 {
    let instructions = parse_instructions(movement_instructions);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // Starting positions
    let mut rope = Rope {
        head: KnotPosition { row: 0, column: 0 },
        tail: KnotPosition { row: 0, column: 0 },
    };

    visited.insert((rope.tail.row, rope.tail.column));

    for instruction in instructions {
        let direction = instruction.0;
        let distance = instruction.1;

        // Move one step at a time
        for _ in 0..distance {
            rope = rope.move_head(direction);
            // Update visited
            visited.insert((rope.tail.row, rope.tail.column));
        }
    }
    visited.len() as i32
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rope {
    head: KnotPosition,
    tail: KnotPosition,
}

impl Rope {
    fn move_head(&self, direction: Direction) -> Rope {
        let new_head = match direction {
            Up => { KnotPosition { row: self.head.row + 1, column: self.head.column } }
            Down => { KnotPosition { row: self.head.row - 1, column: self.head.column } }
            Right => { KnotPosition { row: self.head.row, column: self.head.column + 1 } }
            Left => { KnotPosition { row: self.head.row, column: self.head.column - 1 } }
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
                KnotPosition { row: self.head.row, column: new_column }
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
                KnotPosition { row: new_row, column: new_column }
            }
        } else { self.tail };


        // Return rope
        Rope {
            head: new_head,
            tail: new_tail,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct KnotPosition {
    row: i32,
    column: i32,
}

impl KnotPosition {
    fn is_touching(&self, other: KnotPosition) -> bool {
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

    const MOVEMENTS: &str = indoc! {"R 4
                                     U 4
                                     L 3
                                     D 1
                                     R 4
                                     D 1
                                     L 5
                                     R 2"};

    #[test]
    fn test_tail_visited() {
        let visited = tail_visited(MOVEMENTS.to_string());
        assert_eq!(visited, 13);
    }
}

