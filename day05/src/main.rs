use std::fs;

fn main() {
    let move_instructions: String = fs::read_to_string("./src/move-instructions.txt").expect("Could not open input file");
    let stacks: Vec<Vec<char>> = vec!["SMRNWJVT".chars().collect(), "BWDJQPCV".chars().collect(), "BJFHDRP".chars().collect(), "FRPBMND".chars().collect(), "HVRPTB".chars().collect(), "CBPT".chars().collect(), "BJRPL".chars().collect(), "NCSLTZBW".chars().collect(), "LSG".chars().collect()];
    println!("🏗️ Cratemover 9000 message: {}", move_crates(stacks.clone(), move_instructions.clone(), false));
    println!("🏗️ Cratemover 9001 message: {}", move_crates(stacks, move_instructions, true));
}

fn move_crates(stacks: Vec<Vec<char>>, instructions: String, cratemover_9001: bool) -> String {
    let mut mutable_stacks = stacks;
    for instruction in instructions.lines() {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let num_crates: i32 = parts.get(1).unwrap().parse().unwrap();
        let from: usize = parts.get(3).unwrap().parse().unwrap();
        let to: usize = parts.get(5).unwrap().parse().unwrap();

        if cratemover_9001 {
            let crate_range = (mutable_stacks[from - 1].len() - num_crates as usize)..;
            let mut crates_to_move: Vec<char> = mutable_stacks[from - 1].drain(crate_range).collect();
            mutable_stacks[to - 1].append(&mut crates_to_move);
        } else {
            for _num_crate in 0..num_crates {
                let crate_to_move = mutable_stacks[from - 1].pop().unwrap();
                mutable_stacks[to - 1].push(crate_to_move)
            }
        }
    }

    let mut message: String = "".to_string();
    for stack in mutable_stacks {
        message.push(*stack.last().unwrap());
    }
    message
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const MOVE_INSTRUCTIONS: &str = indoc! {"move 1 from 2 to 1
                                          move 3 from 1 to 3
                                          move 2 from 2 to 1
                                          move 1 from 1 to 2"};

    #[test]
    fn test_cratemover_9000() {
        let stacks: Vec<Vec<char>> = vec!["ZN".chars().collect(), "MCD".chars().collect(), "P".chars().collect()];
        let message = move_crates(stacks, MOVE_INSTRUCTIONS.to_string(), false);
        assert_eq!(message, "CMZ");
    }

    #[test]
    fn test_cratemover_9001() {
        let stacks: Vec<Vec<char>> = vec!["ZN".chars().collect(), "MCD".chars().collect(), "P".chars().collect()];
        let message = move_crates(stacks, MOVE_INSTRUCTIONS.to_string(), true);
        assert_eq!(message, "MCD");
    }
}
