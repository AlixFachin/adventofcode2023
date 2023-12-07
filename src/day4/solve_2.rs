use regex::Regex;
use std::{ fs, collections::HashSet };

const FILEPATH: &str = "src/day4/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut copies_hash = vec![0;line_array.len()+1];

    for line_input in &line_array {
        let re: Regex = Regex::new(r"^Card(?: )+(\d+):((?:\d| )*)\|((?:\d| )*)$").unwrap();
        let matches = re.captures(line_input).unwrap();
        // println!("Line is: {}\n Captures are: {:?}",&line_input, &matches);
        let card_index = matches[1].parse::<usize>().unwrap();
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
                card_score = card_score + 1;
            }
        }
        println!("Card {} has for winning score {}", card_index, card_score);
        // Unlike problem 1, now we add copies of following cards
        let nr_copies = copies_hash[card_index] + 1;
        for i in 1..=card_score {
            copies_hash[card_index + i] = copies_hash[card_index + i] + nr_copies;
        }

    }
    println!("Score array is {:?}", &copies_hash);
    println!("Answer is {}", &copies_hash[1..=line_array.len()].into_iter().fold(0, |acc, x| x + acc + 1 ));
}
