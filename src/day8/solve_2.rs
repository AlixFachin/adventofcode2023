use regex::Regex;
use std::{ fs, collections::HashMap };

const FILEPATH: &str = "src/day8/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let directions: Vec<char> = line_array[0].chars().collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let re_code = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
    let mut current_states: Vec<&str> = vec![];
    for i in 2..line_array.len() {
        let matches = re_code.captures(line_array[i]).unwrap();
        let origin = matches.get(1).unwrap().as_str();
        if origin.ends_with('A') {
            current_states.push(origin);
        }
        let left = matches.get(2).unwrap().as_str();
        let right = matches.get(3).unwrap().as_str();
        nodes.insert(origin, (left, right));
    }

    let mut steps = 0;    

    while current_states.iter().filter(|s| !(*s).ends_with('Z') ).count() > 0  {
        println!("Step {} - Current states are : {:?}", steps, current_states);
        let decision = directions[steps % directions.len()];
        for i in 0..current_states.len() {
            let (left, right) = nodes.get(current_states[i]).unwrap();
            if decision == 'L' {
                current_states[i] = left;
            } else {
                current_states[i] = right;
            }
        }
        steps = steps + 1;
    }

    println!("Number of steps is: {}", steps);

}
