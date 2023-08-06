fn main() {
    // The lines of the input file
    let rows: Vec<&str> = include_str!("./input.txt").lines().collect();
    // How many bits are there in the binary strings (all rows have the same number of bits so just look at the first row)?
    let num_columns: usize = rows[0].len();
    // Map each row to a vector of the bits/characters
    let grid: Vec<Vec<char>> = rows.iter().map(|value| value.chars().collect()).collect();

    // Will hold the most common bit in each column
    let mut gamma_binary_string = String::new();
    // Will hold the least common bit in each column
    let mut epsilon_binary_string = String::new();

    // Will hold the rows that meet the bit criteria
    let mut oxygen_generator_grid = grid.clone();
    let mut co2_scrubber_grid = grid.clone();

    for column_number in 0..(num_columns) {
        // The values/bits for just the currently iterated column
        let column_values: Vec<char> = grid.iter().map(|row| row[column_number]).collect();

        // How many bits are "0"?
        let num_zero_bits = column_values
            .iter()
            .filter(|&x| x.to_string() == "0")
            .count();

        // How many bits are "1"?
        let num_one_bits = column_values
            .iter()
            .filter(|&x| x.to_string() == "1")
            .count();

        // More 1 bits than 0 bits?
        if num_one_bits >= num_zero_bits {
            gamma_binary_string.push_str("1");
            epsilon_binary_string.push_str("0");
        } else {
            gamma_binary_string.push_str("0");
            epsilon_binary_string.push_str("1");
        }

        // Oxygen generator
        let oxygen_column_values: Vec<char> = oxygen_generator_grid
            .iter()
            .map(|row| row[column_number])
            .collect();

        let oxygen_num_zero_bits = oxygen_column_values
            .iter()
            .filter(|&x| x.to_string() == "0")
            .count();

        let oxygen_num_one_bits = oxygen_column_values
            .iter()
            .filter(|&x| x.to_string() == "1")
            .count();

        if oxygen_generator_grid.len() > 1 {
            let most_common_bit = if oxygen_num_one_bits >= oxygen_num_zero_bits {
                "1"
            } else {
                "0"
            };

            oxygen_generator_grid = oxygen_generator_grid
                .drain(..)
                .filter(|row| row[column_number].to_string() == most_common_bit)
                .collect();
        }

        // CO2 scrubbing
        let co2_column_values: Vec<char> = co2_scrubber_grid
            .iter()
            .map(|row| row[column_number])
            .collect();

        let co2_num_zero_bits = co2_column_values
            .iter()
            .filter(|&x| x.to_string() == "0")
            .count();

        let co2_num_one_bits = co2_column_values
            .iter()
            .filter(|&x| x.to_string() == "1")
            .count();

        if co2_scrubber_grid.len() > 1 {
            let least_common_bit = if co2_num_zero_bits <= co2_num_one_bits {
                "0"
            } else {
                "1"
            };

            co2_scrubber_grid = co2_scrubber_grid
                .drain(..)
                .filter(|row| row[column_number].to_string() == least_common_bit)
                .collect();
        }
    }

    // Convert the binary strings to decimal values
    let gamma_decimal_value = isize::from_str_radix(&gamma_binary_string, 2).unwrap();
    let epsilon_decimal_value = isize::from_str_radix(&epsilon_binary_string, 2).unwrap();

    let power_consumption = gamma_decimal_value * epsilon_decimal_value;
    println!("Part 1: {power_consumption}");

    // The first (and only remaining) vector contains the characters of the binary string(s)
    let oxygen_generator_binary_string: String = oxygen_generator_grid[0].iter().collect();
    let co2_scrubbing_binary_string: String = co2_scrubber_grid[0].iter().collect();

    let oxygen_generator_decimal_value =
        isize::from_str_radix(&oxygen_generator_binary_string, 2).unwrap();
    let co2_scrubbing_decimal_value =
        isize::from_str_radix(&co2_scrubbing_binary_string, 2).unwrap();

    let life_support = oxygen_generator_decimal_value * co2_scrubbing_decimal_value;
    println!("Part 2: {life_support}");
}
