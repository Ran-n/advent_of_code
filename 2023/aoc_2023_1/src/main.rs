// -*- coding: utf-8 -*-
// -----------------------------------------------------------------------------
//+ Autor:  	Ran#
//+ Creado: 	2023/12/20 17:45:04.029141
//+ Editado:	2023/12/27 22:11:36.093945
// -----------------------------------------------------------------------------

use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> io::Result<()> {
    let file_path: &str = "media/input.txt";
    let mut calibration_value: String = String::new();
    let mut calibration_sum: i32 = 0;

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // finding the first digit
        for c in line.chars() {
            if c.is_digit(10) {
                calibration_value.push(c);
                // finding the second digit
                for c_rev in line.chars().rev() {
                    if c_rev.is_digit(10) {
                        calibration_value.push(c_rev);
                        break
                    }
                }
                //println!("{}", calibration_value);
                calibration_sum += calibration_value.parse::<i32>().unwrap_or_else(|e| panic!("Error: {}", e));
                calibration_value.clear();
                break
            }
        }
    }
    println!("Part 1 result: {}", calibration_sum);
    Ok(())
}

fn part2(verbose: bool) -> io::Result<()> {
    let file_path: &str = "media/input.txt";
    //let file_path: &str = "media/input_part2_test.txt";
    let mut calibration_value: String = String::new();
    let mut calibration_sum: i32 = 0;

    let written_numbers = vec![
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "7ine"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        if verbose {println!("{}", line);}
        for (search, replace) in &written_numbers {
            line = line.replace(search, replace);
        }
        if verbose {println!("{}", line);}
        // finding the first digit
        for c in line.chars() {
            if c.is_digit(10) {
                calibration_value.push(c);
                if verbose {print!("{}", c);}
                // finding the second digit
                for c_rev in line.chars().rev() {
                    if c_rev.is_digit(10) {
                        calibration_value.push(c_rev);
                        if verbose {println!("{}\n", c_rev);}
                        break
                    }
                }
                //println!("{}", calibration_value);
                calibration_sum += calibration_value.parse::<i32>().unwrap_or_else(|e| panic!("Error: {}", e));
                calibration_value.clear();
                break
            }
        }
    }
    println!("Part 2 result: {}", calibration_sum);
    Ok(())
}

fn main() {
    let _ = part1();
    let _ = part2(false);
}
