use std::{ fs, cmp::Ordering, collections::HashMap };

#[derive(PartialEq, Eq, Debug)]
enum Types {
    FiveKind(char),
    FourKind(char),
    FullHouse((char, char)),
    ThreeKind(char),
    TwoPairs((char, char)),
    OnePair(char),
    HighCard,
}

#[derive(Debug)]
struct Hand {
    desc: Vec<char>,
    bid: usize,
    card_type: Option<Types>,
    joker_count: u8,
}

fn get_card_point(a: &char) -> usize {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => String::from(*a).parse::<usize>().unwrap(),
    }
}

fn compare_string(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    if a.len() != b.len() {
        panic!("Trying to compare two strings of different lengths!");
    }

    for (a_i, b_i) in a.iter().zip(b.iter()) {
        if a_i != b_i {
            return get_card_point(a_i).cmp(&get_card_point(b_i));
        }
    }

    panic!("The strings are equal or not build properly");
}

impl Hand {
    fn new(desc: &str, bid: &str) -> Hand {
        if desc.len() != 5 {
            panic!("Cannot build corresponding card - string length is wrong");
        }

        return Hand {
            desc: desc.chars().collect(),
            bid: bid.parse::<usize>().unwrap(),
            card_type: None,
            joker_count: 0,
        };
    }

    fn cmp(&self, other_hand: &Hand) -> Ordering {
        if self.card_type.is_none() || other_hand.card_type.is_none() {
            panic!("No type defined - cannot compare");
        }
        match (self.card_type.as_ref().unwrap(), other_hand.card_type.as_ref().unwrap()) {
            (Types::FiveKind(_a), Types::FiveKind(_b)) => compare_string(&self.desc, &other_hand.desc),
            (Types::FiveKind(_a), _) => Ordering::Greater,
            (_, Types::FiveKind(_a)) => Ordering::Less,
            (Types::FourKind(_a), Types::FourKind(_b)) =>
                compare_string(&self.desc, &other_hand.desc),
            (Types::FourKind(_a), _) => Ordering::Greater,
            (_, Types::FourKind(_a)) => Ordering::Less,
            (Types::FullHouse((_a1, _a2)), Types::FullHouse((_b1, _b2))) =>
                compare_string(&self.desc, &other_hand.desc),
            (Types::FullHouse((_a1, _a2)), _) => Ordering::Greater,
            (_, Types::FullHouse((_a1, _a2))) => Ordering::Less,
            (Types::ThreeKind(_a), Types::ThreeKind(_b)) =>
                compare_string(&self.desc, &other_hand.desc),
            (Types::ThreeKind(_a), _) => Ordering::Greater,
            (_, Types::ThreeKind(_b)) => Ordering::Less,
            (Types::TwoPairs((_a1, _a2)), Types::TwoPairs((_b1, _b2))) =>
                compare_string(&self.desc, &other_hand.desc),
            (Types::TwoPairs((_a1, _a2)), _) => Ordering::Greater,
            (_, Types::TwoPairs((_b1, _b2))) => Ordering::Less,
            (Types::OnePair(_a), Types::OnePair(_b)) =>
                compare_string(&self.desc, &other_hand.desc),
            (Types::OnePair(_a), _) => Ordering::Greater,
            (_, Types::OnePair(_a)) => Ordering::Less,
            _ => compare_string(&self.desc, &other_hand.desc),
        }
    }

    fn parse_type(&mut self) {
        // Will read the description and attribute the type
        let mut card_hash: HashMap<char, usize> = HashMap::new();
        for c in &self.desc {
            if *c == 'J' {
                self.joker_count = self.joker_count  + 1;
            } else {
                match card_hash.get(&c) {
                    Some(card_count) => card_hash.insert(*c, card_count + 1),
                    None => card_hash.insert(*c, 1),
                };
            }
        }

        // Question 2 modification: According to the number of jokers we add the 

        let mut cards_array: Vec<(&char, &usize)> = card_hash.iter().collect();
        cards_array.sort_by(|a, b| a.1.cmp(b.1));
        cards_array.reverse();

        // the best hand we can make with the joker is increasing the biggest number of cards
        // we already have
        if cards_array.len() == 0 {
            cards_array.push((&'J',&0));
        } 
        let new_number = cards_array[0].1 + self.joker_count as usize;
        cards_array[0].1 = &new_number;
    

        let type_code = cards_array
            .iter()
            .fold(String::from(""), |acc, (_, n)| format!("{}{}", acc, n));

        match type_code.as_str() {
            "11111" => {
                self.card_type = Some(Types::HighCard);
            }
            "2111" => {
                self.card_type = Some(Types::OnePair(*cards_array[0].0));
            }
            "221" => {
                self.card_type = Some(Types::TwoPairs((*cards_array[0].0, *cards_array[1].0)));
            }
            "311" => {
                self.card_type = Some(Types::ThreeKind(*cards_array[0].0));
            }
            "32" => {
                self.card_type = Some(Types::FullHouse((*cards_array[0].0, *cards_array[1].0)));
            }
            "41" => {
                self.card_type = Some(Types::FourKind(*cards_array[0].0));
            }
            "5" => {
                self.card_type = Some(Types::FiveKind(*cards_array[0].0));
            }
            _ => panic!("Unknown card type code! {}", type_code),
        }
    }
}

#[test]
fn test_card_parse() {

    // Four jokers
    let mut hand_test = Hand::new("AJJJJ","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FiveKind('A'));
    // Three jokers
    hand_test = Hand::new("AAJJJ","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FiveKind('A'));

    hand_test = Hand::new("ATJJJ","5");
    hand_test.parse_type();
    if let Some(Types::FourKind(x)) = hand_test.card_type {
        println!("The joker is replacing: {}", x);
        assert!(true);
    } else {
        assert!(false);
    }    
    // Two jokers
    hand_test = Hand::new("AAAJJ","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FiveKind('A'));

    hand_test = Hand::new("AATJJ","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FourKind('A'));

    hand_test = Hand::new("AT9JJ","5");
    hand_test.parse_type();
    if let Some(Types::ThreeKind(x)) = hand_test.card_type {
        println!("The joker is replacing: {}", x);
        assert!(true);
    } else {
        assert!(false);
    }    

    // One joker
    hand_test = Hand::new("AAAAJ","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FiveKind('A'));

    hand_test = Hand::new("AAA9J","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::FourKind('A'));

    hand_test = Hand::new("AA89J","5");
    hand_test.parse_type();
    assert!(hand_test.card_type.unwrap() == Types::ThreeKind('A'));

    hand_test = Hand::new("AA88J","5");
    hand_test.parse_type();
    if let Some(Types::FullHouse((a,b))) = hand_test.card_type {
        println!("The joker is replacing: {},{}", a,b);
        assert!(true);
    } else {
        assert!(false);
    }

    hand_test = Hand::new("A789J","5");
    hand_test.parse_type();
    if let Some(Types::OnePair(x)) = hand_test.card_type {
        println!("The joker is replacing: {}", x);
        assert!(true);
    } else {
        assert!(false);
    }    
    

}

#[test]
fn test_card_compare() {
    let mut hand_1 = Hand::new("AJAJA","5");
    hand_1.parse_type();
    let mut hand_2 = Hand::new("JJJJJ", "10");
    hand_2.parse_type();

    assert!(hand_1.cmp(&hand_2) == Ordering::Less);

}


pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut hand_list: Vec<Hand> = vec![];
    for line in line_array {
        let z: Vec<&str> = line.split(' ').collect();
        let mut h = Hand::new(z[0], z[1]);
        h.parse_type();
        hand_list.push(h);
    }

    hand_list.sort_by(|h1, h2| h1.cmp(h2));

    let mut total_winnings = 0;
    for i in 0..hand_list.len() {
        total_winnings = total_winnings + (i + 1) * hand_list[i].bid;
    }

    println!("Total winnings: {}", total_winnings);
}
