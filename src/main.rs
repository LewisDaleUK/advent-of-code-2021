use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod depth;

fn main() {
    let depths = read_depths();
    let times_increased = depth::times_depth_increased(depths);
    println!("{}", times_increased);
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