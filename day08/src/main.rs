use std::cmp::max;
use std::fs;
use grid::Grid;

fn main() {
    let trees: String = fs::read_to_string("./day08/src/input.txt").expect("Could not open input file");
    println!("🎄 Visible trees: {}", count_visible_trees(trees.clone()));
    println!("🤩 Max scenic score: {}", max_scenic_score(trees));
}

fn count_visible_trees(input: String) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let columns: usize = lines[0].chars().count();

    let mut trees: Grid<i32> = Grid::new(0, 0);
    let mut visible: Grid<i32> = Grid::new(rows, columns);

    for line in lines {
        let row = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        trees.push_row(row)
    }

    // From right
    visible = flag_visible(trees.clone(), visible);

    // From top
    visible = flag_visible(trees.transpose(), visible.transpose()).transpose();
    
    // From bottom
    let bottom_up = reverse_rows(trees.clone()).transpose();
    let visible_reversed = reverse_rows(visible).transpose();
    visible = reverse_rows(flag_visible(bottom_up.clone(), visible_reversed).transpose());
        
    // From left
    let mirrored_columns = mirror_columns(trees.clone());
    let visible_mirrored = mirror_columns(visible);
    visible = flag_visible(mirrored_columns, visible_mirrored);
    visible.iter().sum()
}

fn max_scenic_score(input: String) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let columns: usize = lines[0].chars().count();
    let mut trees: Grid<i32> = Grid::new(0, 0);
    for line in lines {
        let row = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        trees.push_row(row)
    }

    let mut row = 0;
    let mut col = 0;
    let mut max_score = 0;
    for tree in trees.iter() {
        let mut righ_score = 0;
        let mut left_score = 0;
        let mut down_score = 0;
        let mut up_score = 0;

        let mut column_idx = col;
        // Right
        loop {
            column_idx += 1;
            if column_idx == columns {
                break
            }

            if trees[row][column_idx] < *tree {
                righ_score += 1;
            } else {
                righ_score += 1;
                break;    
            }
        }
        
        // Left
        column_idx = col;
        loop {
            if column_idx == 0 || column_idx == columns {
                break;
            }
            column_idx -= 1;
            if trees[row][column_idx] < *tree {
                left_score += 1;
            } else {
                left_score += 1;
                break;   
            }
        }
        
        // Down 
        let mut row_idx = row;
        loop {
            row_idx += 1;
            if row_idx == rows {
                break;
            }
            if trees[row_idx][col] < *tree {
                down_score += 1;
            } else {
                down_score += 1;
                break;
            }
        }
        
        // Up
        row_idx = row;
        loop {
            if row_idx == 0 || row_idx == columns {
                break;
            }
            row_idx -= 1;
            if trees[row_idx][col] < *tree {
                up_score += 1;
            } else {
                up_score += 1;
                break;
            }
        }

        let total_score = righ_score * left_score * up_score * down_score;
        max_score = max(total_score, max_score);
        
        col += 1;
        if col % columns == 0 {
            row += 1;
            col = 0;
        }
    }
    max_score
}

fn reverse_rows(mut grid: Grid<i32>) -> Grid<i32> {
    let mut reversed_grid: Grid<i32> = Grid::new(0, 0);
    while grid.rows() > 0 {
        let next_row = grid.pop_row().unwrap();
        reversed_grid.push_row(next_row)
    }
    reversed_grid
}

fn mirror_columns(mut grid: Grid<i32>) -> Grid<i32> {
    let mut mirrored_columns: Grid<i32> = Grid::new(0, 0);
    while grid.cols() > 0 {
        let next_col = grid.pop_col().unwrap();
        mirrored_columns.push_col(next_col)
    }
    mirrored_columns
}

fn flag_visible(trees: Grid<i32>, mut visible_trees: Grid<i32>) -> Grid<i32> {
    let mut row = 0;
    let mut column = 0;
    let mut highest_tree = -1;
    for tree in trees.iter() {
        if *tree > highest_tree {
            visible_trees[row][column] = 1;
            highest_tree = *tree;
        }
        column += 1;

        if column % trees.cols() == 0 {
            // New row
            row += 1;
            column = 0;
            highest_tree = -1;
        }
    }
    visible_trees
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TREES: &str = indoc! {"30373
                                 25512
                                 65332
                                 33549
                                 35390"};

    #[test]
    fn test_count_visible_trees() {
        let visible_trees = count_visible_trees(TREES.to_string());
        assert_eq!(visible_trees, 21);
    }
    
    #[test]
    fn test_scenic_score() {
        let max_scenic_score = max_scenic_score(TREES.to_string());
        assert_eq!(max_scenic_score, 8);
    }
}
