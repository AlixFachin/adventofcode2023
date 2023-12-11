use std::fs;

const FILEPATH: &str = "src/day9/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();
    let mut sum_extrapolated: i64 = 0;
    for line in line_array {
        let mut sequence: Vec<i64> = line.split(' ').into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
        let mut sequences: Vec<Vec<i64>> = vec![];
        while sequence.iter().filter(|x| **x != 0).count() > 0 {
            sequences.push(sequence.clone());
            // creating the next sequence
            for i in 0..sequence.len()-1 {
                sequence[i] = sequence[i+1] - sequence[i];
            }
            sequence.pop();
            println!("Sequence = {:?}", &sequence);
        }
        // Now going back up the chain to forecast the last element
        
        let mut last_elt_previous_seq: i64 = 0;
        for i in (0..sequences.len()).rev() {
            let seq = &sequences[i];
            println!("Subtracting {} from {}", last_elt_previous_seq, seq[0]);
            last_elt_previous_seq =  seq[0] - last_elt_previous_seq;
        }
        println!("The final sequence forecast is {}", last_elt_previous_seq);
        sum_extrapolated = sum_extrapolated + last_elt_previous_seq;

    }
    println!("------- The answer is {}", sum_extrapolated);

}
