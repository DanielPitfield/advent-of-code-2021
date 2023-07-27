fn main() { 
    let values: Vec<u16> =  
    // Include/Read the file as a string
    include_str!("./Day1.txt")
    // Get string slices (of each line)
    .lines()
    // Parse each line's value to number
    .map(|line| line.parse().unwrap())
    // Convert into more usable collection of a vector
    .collect();
    
    part1(&values);
    part2(&values);
}

fn part1(values: &Vec<u16>) {
    // Get subslices (pairs)
    let pairs = values.windows(2);

    // Count the occurences of the second value in the pair being greater than the first value
    let mut counter: u16 = 0;
    for pair in pairs {
        if pair[1] > pair[0] { counter += 1; }
    }

    println!("{}", counter);
}

fn part2(values: &Vec<u16>) {
    // Get three value sliding windows
    let windows = values.windows(3);
    // Sum together the three values within each window
    let sum_values: Vec<u16> = windows.map(|window| window.iter().sum()).collect();

    let pairs = sum_values.windows(2);

    let mut counter: u16 = 0;
    for pair in pairs {
        if pair[1] > pair[0] { counter += 1; }
    }

    println!("{}", counter);
}