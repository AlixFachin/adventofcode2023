use std::fs;

const FILEPATH: &str = "src/day3/input.txt";
const DEBUG : bool = true;

fn is_digit(c: char) -> bool {
    return c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9';
}


fn get_power(number_grid: &Vec<Vec<i32>>, i0: usize, j0: usize) -> i32 {

    let delta_list = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1),(1,-1),(1,0),(1,1)];
    let mut number_list: Vec<i32> = vec![];

    for (di,dj) in delta_list {
        let i2: i32 = i0 as i32 + di;
        let j2 : i32 = j0 as i32 + dj;
        if i2 >= 0 && (i2 as usize) < number_grid.len() && j2 >= 0 && (j2 as usize) < number_grid[0].len() {
            let i_us = i2 as usize;
            let j_us = j2 as usize;
            if number_grid[i_us][j_us] != 0 && !number_list.contains(&number_grid[i_us][j_us]) {
                number_list.push(number_grid[i_us][j_us]);
            }
        }
    }
    if number_list.len() == 2 {
        return number_list[0]*number_list[1];
    } else {
        return 0;
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut number_grid: Vec<Vec<i32>> = Vec::new();

    for line in line_array {
        let mut line_vec: Vec<char> = vec![];
        for c in line.chars() {
            line_vec.push(c);
        }
        grid.push(line_vec);
        number_grid.push(vec![0;grid[0].len()]);
    }

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
                for k in j0..j {
                    number_grid[i][k] = number.parse::<i32>().unwrap();
                }
            } else {
                j = j + 1;
            }
        }
    }
    
    let mut result = 0;
    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {

            if grid[i][j] == '*' {
                // Test if there are any adjacent numbers
                result = result + get_power(&number_grid, i, j);

            }
        }
    }


    println!("{}", result);

}
