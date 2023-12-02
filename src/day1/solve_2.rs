use std::fs;

const FILEPATH: &str = "src/day1/input.txt";
const DEBUG: bool = true;

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
        _ => None,
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let mut number_total = 0;

    let patterns = [("one","1"),("two","2"),("three","3"),("four","4"),("five","5"),("six","6"),("seven","7"),("eight","8"),("nine","9")];



    let line_array: Vec<&str> = contents.split("\n").collect();
    for line in line_array {
        
        // THE ISSUE:
        // skdpdfqtglzvlpqxp3twonelcr -> we should not replace the "two"

        let mut min_index = line.len();
        let mut min_pattern = "";
        let mut min_digit = "";

        for (p,x) in patterns {
            if let Some(index) = line.find(p) {
                if index < min_index {
                    min_index = index;
                    min_pattern = p;
                    min_digit = x;
                }
            }
        }
       

        let mut max_index = 0;
        let mut max_pattern = "";
        let mut max_digit = "";

        for (p,x) in patterns {
            if let Some(index) = line.rfind(p) {
                if index >= max_index {
                    max_index = index;
                    max_pattern = p;
                    max_digit = x;
                }
            }
        }

        let replaced_left = line.replacen(min_pattern, min_digit,1);
        let replaced_right = line.replace(max_pattern, max_digit);
        
        let mut char_code = 0;
        for c in replaced_left.chars() {
            if let Some(d) = get_digit(c) {
                char_code = 10 * d;
                break;
            }
        }
        for c in replaced_right.chars().rev() {
            if let Some(d) = get_digit(c) {
                char_code = char_code + d;
                break;
            }
        }
        if DEBUG {
            println!("Char is {} for line {}", char_code, line);
        }
        number_total = number_total + char_code;
    }

    println!("Number total is: {}", number_total);
}
