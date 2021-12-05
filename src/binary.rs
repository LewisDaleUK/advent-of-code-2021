pub fn power_consumption(report: Vec<String>) -> i32 {
    let mut epsilon = String::from("");
    let mut gamma = String::from("");

    for i in 0..report[0].len() {
        gamma.push(most_common_bit_at(&report, i));
        epsilon.push(least_common_bit_at(&report, i));   
    }

    let epsilon_val = i32::from_str_radix(epsilon.as_str(), 2).unwrap();
    let gamma_val = i32::from_str_radix(gamma.as_str(), 2).unwrap();
    return epsilon_val * gamma_val;
}

fn count_bit_incidences(report: &Vec<String>, index: usize) -> (i32, i32) {
    let mut ones = 0;
    let mut zeroes = 0;

    for line in report {
        match line.chars().nth(index).unwrap() {
            '0' => zeroes += 1,
            '1' => ones += 1,
            _ => ()
        }
    }

    return (zeroes, ones);
}

fn most_common_bit_at(report: &Vec<String>, index: usize) -> char {
    let (zeroes, ones) = count_bit_incidences(report, index);

    if ones > zeroes {
        return '1';
    }

    return '0';
}

fn least_common_bit_at(report: &Vec<String>, index: usize) -> char {
    let (zeroes, ones) = count_bit_incidences(report, index);
    
    if ones < zeroes {
        return '1';
    }

    return '0';
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculates_energy_consumption() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        ];

        let consumption = super::power_consumption(report);

        assert_eq!(consumption, 198);
    }
}