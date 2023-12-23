// -*- coding: utf-8 -*-
// -----------------------------------------------------------------------------
//+ Autor:  	Ran#
//+ Creado: 	2023/12/20 17:45:04.029141
//+ Editado:	2023/12/23 18:29:02.939558
// -----------------------------------------------------------------------------

use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;

fn main() -> io::Result<()> {
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
    println!("{}", calibration_sum);
    Ok(())
}
