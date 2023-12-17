use regex::Regex;
use std::{ fs, collections::HashSet };

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut total_score = 0;
    for line_input in line_array {
        let re: Regex = Regex::new(r"^Card(?: )+(\d+):((?:\d| )*)\|((?:\d| )*)$").unwrap();
        let matches = re.captures(line_input).unwrap();
        // println!("Line is: {}\n Captures are: {:?}",&line_input, &matches);
        let card_index = &matches[1];
        let winning_numbers: Vec<i32> = matches[2]
            .trim()
            .split(" ")
            .map(|x| x.parse::<i32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        let card_numbers: Vec<i32> = matches[3]
            .trim()
            .split(" ")
            .map(|x| x.parse::<i32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        println!("After parse: W= {:?}, my={:?}", winning_numbers, card_numbers);

        let mut winning_hash: HashSet<i32> = HashSet::new();
        for x in winning_numbers {
            winning_hash.insert(x);
        }
        let mut card_score = 0;
        for x in card_numbers {
            if winning_hash.contains(&x) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score = 2 * card_score;
                }
            }
        }
        println!("Card {} has for winning score {}", card_index, card_score);
        total_score = total_score + card_score;
    }
    println!("Total score is {}", total_score);
}
