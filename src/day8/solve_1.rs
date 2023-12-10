use regex::Regex;
use std::{ fs, collections::HashMap };

const FILEPATH: &str = "src/day8/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let directions: Vec<char> = line_array[0].chars().collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let re_code = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
    for i in 2..line_array.len() {
        let matches = re_code.captures(line_array[i]).unwrap();
        let origin = matches.get(1).unwrap().as_str();
        let left = matches.get(2).unwrap().as_str();
        let right = matches.get(3).unwrap().as_str();
        nodes.insert(origin, (left, right));
    }

    let mut steps = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let (left, right) = nodes.get(current_node).unwrap();
        let decision = directions[steps % directions.len()];
        if decision == 'L' {
            current_node = left;
        } else {
            current_node = right;
        }
        steps = steps + 1;
    }

    println!("Number of steps is: {}", steps);

}
