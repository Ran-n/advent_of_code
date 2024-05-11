use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn dict_1(file_path: &str) -> io::Result<()> {
    let mut color_cubes: HashMap<u32, Vec<(String, i32)>> = HashMap::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        // partition at :
        let initial_partition: Vec<String> = line
            .split(": ")
            .map(String::from)
            .collect();

        // getting the id
        let game_id: i32 = initial_partition[0].split(" ").skip(1).next().unwrap_or("").to_string()
            .parse::<i32>().unwrap_or_else(|e| panic!("Error: {}", e));

        // partition by ; of the :'s right part
        let games: Vec<String> = initial_partition[1]
            .split("; ")
            .map(String::from)
            .collect();

        for game in &games {
            let rounds: Vec<String> = game
                .split(", ")
                .map(String::from)
                .collect();
            for round in &rounds {
                println!("{}", round);
            }
            //println!("{}", game);
        }

        //println!("{}", line.split(":").skip(0).next().unwrap_or("").split(" ").skip(1).next().unwrap_or("").to_string());
    }

    Ok(())
}

fn part1(reds: i32, greens: i32, blues: i32) {
    let file_path: &str = "media/input.txt";
    dict_1(file_path);
    println!("Part 1 [{}r|{}g|{}b] result: {}", reds, greens, blues, 1);
}

fn main() {
    let _ = part1(12, 13, 14);
}
