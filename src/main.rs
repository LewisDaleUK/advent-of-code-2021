use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

mod depth;
mod position;
mod binary;
mod bingo;

fn main() {
    let depths = read_depths();
    let times_increased = depth::times_depth_increased(depths);
    println!("Times Depth Increased: {}", times_increased);

    let instructions = read_directions();
    let (horizontal, depth) = position::calculate_position(instructions, 0, 0, 0);
    println!("Final Position: {}", horizontal * depth);

    let report = get_diagnostic_report();
    let consumption = binary::life_support_rating(report.to_vec());
    let life_support = binary::life_support_rating(report.to_vec());

    println!("Power Consumption: {}", consumption);
    println!("Life Support: {}", life_support);

    let (numbers, boards) = get_bingo_game();
    let last_score = bingo::find_last_winning_score(boards, numbers);
    println!("Last Wining Bingo Score: {}", last_score);
}

fn get_diagnostic_report() -> Vec<String> {
    if let Ok(lines) = read_lines("./files/day_3_input.txt") {
        let mut report: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(l) = line {
                report.push(l);
            }
        }

        return report;
    } else {
        return vec![];
    }
}

fn get_bingo_game() -> (Vec<i32>, Vec<bingo::Board>) {
    if let Ok(mut lines) = read_lines("./files/day_4_input.txt") {
        let mut numbers: Vec<i32> = Vec::new();
        let mut boards: Vec<bingo::Board> = Vec::new();

        if let Ok(number_line) = lines.next().unwrap() {
            numbers = number_line.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
        }

        for mut chunk in &lines.chunks(6) {
            chunk.next(); // Throw away the whitespace
            let mut grid: [[i32; 5]; 5] = Default::default();

            for i in 0..5 {
                let line = chunk.next().unwrap();
                if let Ok(l) = line {
                    let board_line: Vec<i32> = l.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

                    for j in 0..5 {
                        grid[i][j] = board_line[j];
                    }
                }
            }

            boards.push(bingo::Board::new(grid));
        }

        return (numbers, boards);
    }

    return (vec![], vec![]);
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