use std::fs;

const MAX_BLUE: u32 = 14;
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut result = 0;
    // Example:  Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    for (i, line) in line_array.iter().enumerate() {
        let game_descriptor = line.split(':').collect::<Vec<&str>>()[1];
        let tries: Vec<&str> = game_descriptor.split(';').collect();
        let mut is_possible: bool = true;
        for game_try in tries {
            let color_list : Vec<&str> = game_try.split(',').collect();
            for color in color_list {
                let try_color_desc: Vec<&str> = color.trim().split(' ').collect();
                let num_dices: u32 = try_color_desc[0].parse().unwrap();
                let color_name = try_color_desc[1].trim();
                match color_name {
                    "blue" => {
                        if num_dices > MAX_BLUE { is_possible = false}
                    },
                    "red" => {
                        if num_dices > MAX_RED { is_possible = false}
                    },
                    "green" => {
                        if num_dices > MAX_GREEN { is_possible = false}
                    },
                    _ => panic!("Unknown color on game {}",i+1),
                }
            }
        }
        if is_possible {
            result = result + i + 1;
        }
    }

    println!("The result is {}", result);

}
