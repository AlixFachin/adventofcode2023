use regex::Regex;
use std::fs;

const FILEPATH: &str = "src/day1/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

}
