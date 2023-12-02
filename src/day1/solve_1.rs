use std::fs;

const FILEPATH: &str = "src/day1/input.txt";
const DEBUG: bool = false;

fn get_digit(c: char) -> Option<i32> {

    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    }

}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let mut number_total = 0;

    let line_array: Vec<&str> = contents.split("\n").collect();
    for line in line_array {
        
        let mut char_code = 0;
        for c in line.chars() {
            if let Some(d) = get_digit(c) {                
                char_code = 10 * d;
                break;
            }
        }   
        for c in line.chars().rev() {
            if let Some(d) = get_digit(c) {
                char_code = char_code + d;
                break;
            }
        }
        if DEBUG { println!("Char is {} for line {}", char_code, line)}
        number_total = number_total + char_code;
    }

    println!("Number total is: {}", number_total);

}
