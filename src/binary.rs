pub fn power_consumption(report: Vec<String>) -> i32 {
    let mut epsilon = String::from("");
    let mut gamma = String::from("");

    for i in 0..report[0].len() {
        gamma.push(most_common_bit_at(&report, i, '1'));
        epsilon.push(least_common_bit_at(&report, i, '0'));   
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

fn most_common_bit_at(report: &Vec<String>, index: usize, prefer: char) -> char {
    let (zeroes, ones) = count_bit_incidences(report, index);

    if ones == zeroes {
        return prefer;
    }

    if ones > zeroes {
        return '1';
    }

    return '0';
}

fn least_common_bit_at(report: &Vec<String>, index: usize, prefer: char) -> char {
    let (zeroes, ones) = count_bit_incidences(report, index);
    
    if ones == zeroes {
        return prefer;
    }

    if ones < zeroes {
        return '1';
    }

    return '0';
}

fn oxygen_generator_rating(report: &Vec<String>) -> String {
    let mut oxygen_generator = report.to_vec();

    for i in 0..oxygen_generator[0].len() {
        let most_common = most_common_bit_at(&oxygen_generator, i, '1');
        
        oxygen_generator = oxygen_generator
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == most_common)
            .collect();

        if oxygen_generator.len() == 1 {
            break;
        }
    }

    return String::from(oxygen_generator[0].as_str());
}

fn co2_scrubber_rating(report: &Vec<String>) -> String {
    let mut scrubber_rating = report.to_vec();

    for i in 0..scrubber_rating[0].len() {
        let least_common = least_common_bit_at(&scrubber_rating, i, '0');

        scrubber_rating = scrubber_rating
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == least_common)
            .collect();

        if scrubber_rating.len() == 1 {
            break;
        }
    }

    return String::from(scrubber_rating[0].as_str());
}

pub fn life_support_rating(report: Vec<String>) -> i32 {
    let oxygen_generator = oxygen_generator_rating(&report);
    let scrubber_rating = co2_scrubber_rating(&report);


    let oxygen_generator_val = i32::from_str_radix(oxygen_generator.as_str(), 2).unwrap();
    let scrubber_rating_val = i32::from_str_radix(scrubber_rating.as_str(), 2).unwrap();

    return oxygen_generator_val * scrubber_rating_val;
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

    #[test]
    fn calculates_life_support_rating() {
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

        let rating = super::life_support_rating(report);

        assert_eq!(rating, 230);
    }
}