use regex::Regex;
use std::fs;

// struct Tile {
//     code: char,
//     coords: (usize, usize),
//     map_height: usize,
//     map_width: usize,
// }

// impl Tile {
//     fn new(code: char, coords: (usize, usize), height: usize, width: usize) -> Tile {
//         return Tile {
//             code: code,
//             coords: coords,
//             map_height: height,
//             map_width: width,
//         };
//     }

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    // let re_cd: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    // if re_cd.is_match(line_input) {
    //     let matches = re_cd.captures(line_input).unwrap();
    //     let directory_name = matches.get(1).map_or("", |x| x.as_str());
    //           return Some(CommandLine::ChangeDir(String::from(directory_name)));
    // }
}
