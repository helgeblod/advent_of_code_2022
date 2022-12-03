use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("./src/rucksacks.txt").expect("Could not open rucksack file");
    println!("🎒priority sum is {}", sum_rucksack_priorities(&rucksacks));
    println!("🎒badge sum is {}", sum_badges(&rucksacks));
}

fn sum_badges(rucksacks: &str) -> i32 {
    let mut list = rucksacks.lines().collect::<Vec<&str>>();
    let mut badges: Vec<char> = Vec::new();
    while !list.is_empty() {
        // Get 3 items 
        let member1 = list.pop().expect("Could not get inventory from member1");
        let member2 = list.pop().expect("Could not get inventory from member2");
        let member3 = list.pop().expect("Could not get inventory from member3");

        // Find common elements
        for item in member1.chars() {
            if member2.contains(item) && member3.contains(item) {
                badges.push(item);
                break;
            }
        }
    }

    calc(badges)
}

fn sum_rucksack_priorities(rucksacks: &str) -> i32 {
    let mut total = 0;
    for rucksack in rucksacks.lines() {
        // Split into compartments
        let compartments = rucksack.split_at(rucksack.len() / 2);
        let compartment1 = compartments.0.chars().collect::<Vec<char>>();
        let compartment2 = compartments.1.chars().collect::<Vec<char>>();

        let mut common_elements = Vec::new();

        // Find common elements
        for element in compartment1 {
            if compartment2.contains(&element) { common_elements.push(element) }
        }
        common_elements.dedup();
        total += calc(common_elements)
    }
    total
}

fn calc(items: Vec<char>) -> i32 {
    let mut priorities = ('a'..='z').into_iter().collect::<Vec<char>>();
    priorities.append(&mut ('A'..='Z').into_iter().collect::<Vec<char>>());
    let mut priority_sum: i32 = 0;
    for common_element in items {
        let score = priorities.iter().position(|&c| c == common_element).unwrap();
        priority_sum += score as i32 + 1;
    }
    priority_sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::{sum_badges, sum_rucksack_priorities};

    // use super::*;
    const RUCKSACKS: &str = indoc! {"vJrwpWtwJgWrhcsFMMfFFhFp
                                     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                                     PmmdzqPrVvPwwTWBwg
                                     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                                     ttgJtRGJQctTZtZT
                                     CrZsJsPPZsGzwwsLwLmpwMDw"};

    #[test]
    fn test_priority_calculation() {
        assert_eq!(sum_rucksack_priorities(RUCKSACKS), 157);
    }

    #[test]
    fn test_badges() {
        assert_eq!(sum_badges(RUCKSACKS), 70);
    }
}
