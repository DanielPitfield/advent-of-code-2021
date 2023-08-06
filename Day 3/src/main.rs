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

            oxygen_generator_grid = oxygen_generator_grid
                .clone()
                .into_iter()
                .filter(|row| row[column_number].to_string() == "1")
                .collect();

            co2_scrubber_grid = co2_scrubber_grid
                .clone()
                .into_iter()
                .filter(|row| row[column_number].to_string() == "0")
                .collect();
        } else {
            gamma_binary_string.push_str("0");
            epsilon_binary_string.push_str("1");

            oxygen_generator_grid = oxygen_generator_grid
                .clone()
                .into_iter()
                .filter(|row| row[column_number].to_string() == "0")
                .collect();

            co2_scrubber_grid = co2_scrubber_grid
                .clone()
                .into_iter()
                .filter(|row| row[column_number].to_string() == "1")
                .collect();
        }
    }

    // Convert the binary strings to decimal values
    let gamma_decimal_value = isize::from_str_radix(&gamma_binary_string, 2).unwrap();
    let epsilon_decimal_value = isize::from_str_radix(&epsilon_binary_string, 2).unwrap();

    let power_consumption = gamma_decimal_value * epsilon_decimal_value;
    println!("Part 1: {power_consumption}");
}
