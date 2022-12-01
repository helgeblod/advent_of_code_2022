use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/calories.txt")?;
    let reader = BufReader::new(file);
    let mut calorie_counts = Vec::new();
    let mut current_elf = 0;
    for line in reader.lines() {
        let value = line.expect("Error reading line");
        if value.is_empty() {
            calorie_counts.push(current_elf);
            current_elf = 0;
        } else {
            let calorie_amount: i32 = value.parse().unwrap();
            current_elf += calorie_amount;
        }
    }
    
    calorie_counts.sort();
    let top1 = calorie_counts.pop().expect("No more calories");
    let top2 = calorie_counts.pop().expect("No more calories");
    let top3 = calorie_counts.pop().expect("No more calories");
    println!("🥇🧝 carrying: {}", top1);
    println!("🥈🧝 carrying: {}", top2);
    println!("🥉🧝 carrying: {}", top3);
    println!("Total: {}", top1 + top2 + top3);
    Ok(())
}
