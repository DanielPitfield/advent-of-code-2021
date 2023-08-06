fn main() {
    let rows: Vec<&str> = include_str!("./input.txt").lines().collect();

    part1(&rows);
}

fn part1(rows: &Vec<&str>) {
    let mut gamma_binary_string = String::new();
    let mut epsilon_binary_string = String::new();

    // How many bits are there in the binary strings (all rows have the same number of bits so just look at the first row)?
    let num_columns = rows[0].split("").count();
    // Map each row to a vector of the bits/characters
    let grid: Vec<Vec<&str>> = rows.iter().map(|value| value.split("").collect()).collect();

    for column_number in 0..(num_columns - 1) {
        // The values/bits for just the currently iterated column
        let column_values: Vec<&str> = grid.iter().map(|row| row.get(column_number)).collect();

        // More 1 bits than 0 bits?
        if column_values.iter().filter(|&x| x == "1").count() >= column_values.iter().count() / 2 {
            gamma_binary_string.push_str("1");
            epsilon_binary_string.push_str("0");
        } else {
            gamma_binary_string.push_str("0");
            epsilon_binary_string.push_str("1");
        }
    }

    // Convert the binary strings to decimal values
    let gamma_decimal_value = isize::from_str_radix(&gamma_binary_string, 2).unwrap();
    let epsilon_decimal_value = isize::from_str_radix(&epsilon_binary_string, 2).unwrap();

    let power_consumption = gamma_decimal_value * epsilon_decimal_value;
    println!("Part 1: {power_consumption}");
}
