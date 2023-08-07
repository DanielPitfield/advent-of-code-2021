fn main() {
    let mut rows: Vec<&str> = include_str!("./input.txt")
        .lines()
        // Ignore the empty lines (between boards)
        .filter(|row| row.len() > 0)
        .collect();

    // The first line holds all the bingo numbers (also remove from rows)
    let bingo_numbers = rows.remove(0);
}
