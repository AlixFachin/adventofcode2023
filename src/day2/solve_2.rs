use std::fs;

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut power = 0;
    // Example:  Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    for (i, line) in line_array.iter().enumerate() {
        let game_descriptor = line.split(':').collect::<Vec<&str>>()[1];
        let tries: Vec<&str> = game_descriptor.split(';').collect();
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for game_try in tries {
            let color_list : Vec<&str> = game_try.split(',').collect();
            for color in color_list {
                let try_color_desc: Vec<&str> = color.trim().split(' ').collect();
                let num_dices: u32 = try_color_desc[0].parse().unwrap();
                let color_name = try_color_desc[1].trim();
                match color_name {
                    "blue" => {
                        min_blue = min_blue.max(num_dices);
                    },
                    "red" => {
                        min_red = min_red.max(num_dices);
                    },
                    "green" => {
                        min_green = min_green.max(num_dices);
                    },
                    _ => panic!("Unknown color on game {}",i+1),
                }
            }
        }
        power = power + (min_blue*min_red*min_green);
    }
    println!("The result is {}", power);

}
