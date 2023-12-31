use regex::Regex;
use std::fs;

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let re_extract_digits = Regex::new(r"\s+(\d+)").unwrap();
    let matches_1 = re_extract_digits.find_iter(line_array[0]);
    let matches_2 = re_extract_digits.find_iter(line_array[1]);
    let durations: Vec<usize> = matches_1
        .map(|x| x.as_str().trim().parse::<usize>().unwrap())
        .collect();
    let records: Vec<usize> = matches_2
        .map(|x| x.as_str().trim().parse::<usize>().unwrap())
        .collect();

    let races: Vec<(usize, usize)> = durations
        .iter()
        .zip(records.iter())
        .map(|(x, y)| (*x, *y))
        .collect();

    let mut total_records = 1;
    for (d, r) in races {

        let df = d as f64;
        let rf = r as f64;

        let x1 = (df - (df*df - 4.*rf).sqrt())/2.;
        let x2 = (df + (df*df - 4.*rf).sqrt())/2.;
        let x1_u = (x1.ceil().max(0.)).min(df) as i32;
        let x2_u = (x2.ceil().max(0.)).min(df) as i32;
        println!("Solutions are ({},{}) rounded to ({}, {})",x1,x2,x1_u, x2_u);

        // Checking for equality now        
        let mut number_records;

        number_records = 0;
        for i in 0..=d {
            let length_achieved = (d - i) * i;
            if length_achieved > r { number_records = number_records + 1 }
        }
        println!("Number of ways to be race ({},{}) is {}",d,r, number_records);
        
        total_records = total_records * number_records;

    }
    println!("Total ways is: {}", total_records);



}
