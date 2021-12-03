pub fn times_depth_increased(depths: &[i32]) -> i32 {
    let mut last_depth = i32::MAX;
    let mut number_increases: i32 = 0;

    for &depth in depths {
        if (last_depth < depth) {
            number_increases += 1;
        }

        last_depth = depth;
    }
    return number_increases;
}

#[cfg(test)]
mod tests {
    #[test] 
    fn it_returns_7_increases() {
        let depths: [i32; 10] = [
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
        assert_eq!(super::times_depth_increased(&depths), 7);
    }
}