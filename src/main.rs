fn main() {
    // Include/Read the file as a string
    let file_str = include_str!("./Day1.txt");
    // String slices (of each line)
    let lines = file_str.lines();
    // Parse into a number
    let values = lines.map(|line| line.parse().unwrap());
    // Convert into more usable collection of a vector
    let collection = values.collect::<Vec<u16>>();
    // Get subslices (pairs) from the collection
    let pairs = collection.windows(2);

    // Count the occurences of a value being greater than value before it in the collection
    let mut counter: u16 = 0;
    for pair in pairs {
        if pair[1] > pair[0] { counter += 1; }
    }

    println!("{}",counter);
}