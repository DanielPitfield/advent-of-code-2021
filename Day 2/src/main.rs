fn main() { 
    let lines: Vec<&str> =  
    include_str!("./input.txt")
    .lines()
    .collect();
    
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();

        let value: usize =  parts[1].parse().unwrap();
        if parts[0] == "forward" { horizontal_position += value; }
        // Down increases the depth
        else if parts[0] == "down" { depth += value; }
        // Up decreases the depth
        else { depth -= value; }
    }

    let result: usize = depth * horizontal_position;
    println!("Part 1: {result}");
}