pub fn times_depth_increased(depths: Vec<i32>) -> i32 {
    let mut last_sum = i32::MAX;
    let mut number_increases: i32 = 0;

    let mut i = 0;

    while i < depths.len() - 2 {
        if let [a, b, c] = depths[i..i+3] {
            let sum = a + b + c;

            if last_sum < sum {
                number_increases += 1;
            }

            last_sum = sum;
        }
        i += 1;
    }
    
    return number_increases;
}

#[cfg(test)]
mod tests {
    #[test] 
    fn it_returns_7_increases() {
        let depths: Vec<i32> = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        assert_eq!(super::times_depth_increased(depths), 5);
    }
}