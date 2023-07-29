fn main() { 
    let lines: Vec<&str> =  
    include_str!("./input.txt")
    .lines()
    .collect();
    
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in lines {
        // Split to get the direction and the value
        let parts: Vec<&str> = line.split(" ").collect();
        // Parse the value to number
        let value: usize =  parts[1].parse().unwrap();

        // Forward increases horizontal position
        if parts[0] == "forward" { horizontal_position += value; }
        // Down increases the depth
        else if parts[0] == "down" { depth += value; }
        // Up decreases the depth
        else { depth -= value; }
    }

    let result: usize = depth * horizontal_position;
    println!("Part 1: {result}");
}

fn part2(lines: &Vec<&str>) {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let value: usize =  parts[1].parse().unwrap();

        if parts[0] == "forward" { 
            horizontal_position += value;
            // Also now increases depth by aim multiplied by the value
            depth += (aim * value);
        }
        // Increases aim
        else if parts[0] == "down" { aim += value; }
        // Decreases aim
        else { aim -= value; }
    }

    let result: usize = depth * horizontal_position;
    println!("Part 2: {result}");
}