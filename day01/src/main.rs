use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/calories.txt")?;
    let reader = BufReader::new(file);
    let mut elf_calorie_totals = Vec::new();
    let mut current_elf_calories = 0;
    for line in reader.lines() {
        let value = line.expect("Error reading line");
        if value.is_empty() {
            elf_calorie_totals.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calorie_amount: i32 = value.parse().unwrap();
            current_elf_calories += calorie_amount;
        }
    }
    
    elf_calorie_totals.sort();
    let top1 = elf_calorie_totals.pop().expect("No more calories");
    let top2 = elf_calorie_totals.pop().expect("No more calories");
    let top3 = elf_calorie_totals.pop().expect("No more calories");
    println!("🥇🧝 carrying: {}", top1);
    println!("🥈🧝 carrying: {}", top2);
    println!("🥉🧝 carrying: {}", top3);
    println!("Total: {}", top1 + top2 + top3);
    Ok(())
}
