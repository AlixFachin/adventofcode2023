use std::fs;

const FILEPATH: &str = "src/day3/input.txt";
const DEBUG : bool = true;

fn is_digit(c: char) -> bool {
    return c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9';
}

fn is_symbol(c: char) -> bool {
    return c != '.' && !is_digit(c);
}

fn is_adjacent(grid: &Vec<Vec<char>>, i0: usize, j0: usize, j_end: usize) -> bool {

    let col_min = j0.max(1) - 1;
    let col_max = (j_end+1).min(grid[0].len()-1);

    // Checking the row before
    if i0 > 0 {
        for j in col_min..=col_max {
            if is_symbol(grid[i0-1][j]) {
                return true;
            }
        }
    }

    if j0 > 0 && is_symbol(grid[i0][j0-1]) {
        return true;
    }

    if j_end < grid[0].len()-1 && is_symbol(grid[i0][j_end+1]) {
        return true;
    }

    // Checking row below if not last line
    if i0 < grid.len() - 1 {
        for j in col_min..=col_max {
            if is_symbol(grid[i0+1][j]) {
                return true;
            }
        }
    }

    return false;
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in line_array {
        let mut line_vec: Vec<char> = vec![];
        for c in line.chars() {
            line_vec.push(c);
        }
        grid.push(line_vec);
    }

    let mut result = 0;
    for i in 0..grid.len() {

        let row = &grid[i];
        let mut j=0;
        while j < row.len() {
            if is_digit(row[j]) {
                let j0 = j;
                let mut number = String::new();
                while j < row.len() && is_digit(row[j]) {
                    number.push(row[j]);
                    j = j +1;
                }
                if DEBUG { println!("Found number {}", number)}
                // now we need to test if the number is adjacent to a symbol
                if is_adjacent(&grid, i, j0, j-1) {
                    if DEBUG { println!("Adjacent! Adding {} to {}", number, result)}
                    result = result + number.parse::<i32>().unwrap();
                }

            } else {
                j = j + 1;
            }

        }
    }

    println!("{}", result);

}
