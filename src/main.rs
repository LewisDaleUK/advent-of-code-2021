use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod depth;
mod position;

fn main() {
    let depths = read_depths();
    let times_increased = depth::times_depth_increased(depths);
    println!("{}", times_increased);

    let instructions = read_directions();
    let (horizontal, depth) = position::calculate_position(instructions, 0, 0);
    println!("{} final position", horizontal * depth);
}

fn read_directions() -> Vec<(String, i32)> {
    if let Ok(lines) = read_lines("./files/day_2_input.txt") {
        let mut directions: Vec<(String, i32)> = Vec::new();

        for line in lines {
            if let Ok(l) = line {
                let mut split: std::str::Split<&str> = l.split(" ");
                let dir = split.next().unwrap();
                let amt = split.next().unwrap().parse::<i32>().unwrap();

                directions.push((String::from(dir), amt));
            }
        }
        return directions;
    } else {
        return vec![];
    }
}

fn read_depths() -> Vec<i32> {
    if let Ok(lines) = read_lines("./files/depths_input.txt") {
        let mut depths: Vec<i32> = Vec::new();
        
        for line in lines {
            if let Ok(l) = line {
                depths.push(l.parse::<i32>().unwrap());
            }
        }
        return depths;
    } else {
        return vec![];
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}