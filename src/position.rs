pub fn calculate_position(instructions: Vec<(String, i32)>, mut horizontal: i32, mut depth: i32, mut aim: i32) -> (i32, i32) {
    if instructions.len() == 0 {
        return (horizontal, depth);
    }

    let (direction, amount) = &instructions[0];

    match direction.as_str() {
        "forward" => {
            horizontal += amount;
            depth += aim * amount;
        },
        "down" => aim += amount,
        "up" => aim -= amount,
        _ => horizontal = horizontal
    };

    return calculate_position(Vec::from(&instructions[1..]), horizontal, depth, aim);
}

#[cfg(test)]
mod tests {
    #[test] 
    fn calculates_depth_and_horizontal_position() {
        let instructions: Vec<(String, i32)> = vec![
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2)
        ];

        let (horizontal, depth) = super::calculate_position(instructions, 0, 0, 0);

        assert_eq!(horizontal, 15);
        assert_eq!(depth, 60);
    }
}