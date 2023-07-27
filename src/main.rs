fn main() {
    let values =  
    // Include/Read the file as a string
    include_str!("./Day1.txt")
    // Get string slices (of each line)
    .lines()
    // Parse each line's value to number
    .map(|line| line.parse().unwrap())
    // Convert into more usable collection of a vector
    .collect::<Vec<u16>>();

    // Get subslices (pairs) from the collection
    let pairs = values.windows(2);

    // Count the occurences of a value being greater than value before it in the collection
    let mut counter: u16 = 0;
    for pair in pairs {
        if pair[1] > pair[0] { counter += 1; }
    }

    println!("{}", counter);
}