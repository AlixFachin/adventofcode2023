use std::fs;
use clap::{Parser, Subcommand};
use colored::Colorize;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

fn add_folder(n: u8) {
    // Create directory
    let dirpath = format!("src/day{}", n);
    // TEST -> If the folder already exists, then panic (we don't want to overwrite already written code!)
    if std::path::Path::new(&dirpath).exists() {
        panic!("The path already exists - exiting...");
    }

    fs::create_dir(&dirpath).expect("Didn't manage to create the folder!");

    // Copy all the files from the template directory into the proper path
    let all_template_files =
        fs::read_dir("day_template").expect("Didn't manage to read the template file!");
    for template_file in all_template_files {
        let template_file_path = template_file.unwrap().path();
        println!("Copying file {}", template_file_path.display());
        let mut destination_file = std::path::PathBuf::from(&dirpath);
        destination_file.push(&template_file_path.file_name().unwrap());
        fs::copy(&template_file_path, destination_file).expect("Error in file copying!");
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Adds a folder for a new Advent Calendar day
    Add,
    /// Runs the corresponding solving function for the day
    Run {
        #[arg(short, long)]
        /// 1 to run only question 1 problem, 2 to run only question 2, run both if ommitted
        question: Option<u8>,

        #[arg(short, long)]
        test: bool,
    }
}

#[derive(Parser, Debug)]
#[command(author="Alix Fachin", version, about="Executable for Advent of Code 2023")]
struct Arguments {
    #[command(subcommand)]
    cmd: SubCommand,
    
    /// index of the day trying to be solved
    day: u8,

}

fn main() {

    let args = Arguments::parse();
    
    match args.cmd {
        
        SubCommand::Run { question, test } => {
            println!("{}{}","Running the suite ".red(), args.day);
            let day_fn_1: fn(&str);
            let day_fn_2: fn(&str);
            match args.day {                
                1 => {
                    day_fn_1 = day1::solve_1::solve;
                    day_fn_2 = day1::solve_2::solve;
                },
                2 => {
                    day_fn_1 = day2::solve_1::solve;
                    day_fn_2 = day2::solve_2::solve;
                },
                3 => {
                    day_fn_1 = day3::solve_1::solve;
                    day_fn_2 = day3::solve_2::solve;
                },
                4 => {
                    day_fn_1 = day4::solve_1::solve;
                    day_fn_2 = day4::solve_2::solve;
                },
                5 => {
                    day_fn_1 = day5::solve_1::solve;
                    day_fn_2 = day5::solve_2::solve;
                },
                6 => {
                    day_fn_1 = day6::solve_1::solve;
                    day_fn_2 = day6::solve_2::solve;
                },
                7 => {
                    day_fn_1 = day7::solve_1::solve;
                    day_fn_2 = day7::solve_2::solve;
                },
                8 => {
                    day_fn_1 = day8::solve_1::solve;
                    day_fn_2 = day8::solve_2::solve;
                },
                9 => {
                    day_fn_1 = day9::solve_1::solve;
                    day_fn_2 = day9::solve_2::solve;
                },                
                10 => {
                    day_fn_1 = day10::solve_1::solve;
                    day_fn_2 = day10::solve_2::solve;
                },
                // 11 => {
                //     day_fn_1 = day11::solve_1::solve;
                //     day_fn_2 = day11::solve_2::solve;
                // }
                _ => panic!("Module not found!"),
            }
            let problem_input = format!("src/day{}/input{}.txt",args.day, if test { "_test" } else {""});                
            if question.is_none() || question.unwrap() == 1 {
                println!("{},{}","Solving question 1".yellow()," -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-".green());
                day_fn_1(&problem_input);
            }
            if question.is_none() || question.unwrap() == 2 {
                println!("{},{}","Solving question 2".blue()," -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-".green());
                day_fn_2(&problem_input);
            }
        }
        SubCommand::Add => add_folder(args.day),
    }
}
