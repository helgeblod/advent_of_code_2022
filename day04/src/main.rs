use std::cmp::{max, min};
use std::fs;

fn main() {
    let section_inputs: String = fs::read_to_string("./src/section-inputs.txt").expect("Could not open input file");
    println!("🧹 Number of duplicated sections {}", num_contained_sections(&section_inputs, false));
    println!("🧹 Number of overlapping sections {}", num_contained_sections(&section_inputs, true));
}

fn num_contained_sections(section_inputs: &String, include_partial_overlaps: bool) -> i32 {
    let mut num_overlapping_sections = 0;
    for line in section_inputs.lines() {
        let sections = line.split_once(",").expect("Could not parse pair");
        let first_section = sections.0.split_once("-").expect("Could not parse range");
        let second_section = sections.1.split_once("-").expect("Could not parse range");
        let a: i32 = first_section.0.trim().parse().expect("Error parsing number a");
        let b: i32 = first_section.1.trim().parse().expect("Error parsing number b");
        let c: i32 = second_section.0.trim().parse().expect("Error parsing number c");
        let d: i32 = second_section.1.trim().parse().expect(&*format!("Error parsing number: {}", &second_section.1));

        let first_range = a..b;
        let second_range = c..d;
        
        if include_partial_overlaps {
            if a <= d && c <= b {
                num_overlapping_sections += 1;
            }
        } else {
            let merged_range = max(a, c)..min(b, d);
            
            if merged_range == first_range || merged_range == second_range {
                num_overlapping_sections += 1;
            }
        }
    }
    num_overlapping_sections
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::num_contained_sections;

    // use super::*;
    const SECTION_ASSIGNMENTS: &str = indoc! {"2-4, 6-8
                                               2-3, 4-5
                                               5-7, 7-9
                                               2-8, 3-7
                                               6-6, 4-6
                                               2-6, 4-8"};

    #[test]
    fn test_full_overlap() {
        assert_eq!(num_contained_sections(&SECTION_ASSIGNMENTS.to_string(), false), 2);
    }
    
    #[test]
    fn test_partial_overlap() {
        assert_eq!(num_contained_sections(&SECTION_ASSIGNMENTS.to_string(), true), 4);
    }
}
