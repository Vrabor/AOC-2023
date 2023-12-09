use std::collections::HashMap;
use num::Integer;

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

fn get_input(path: &str) -> std::io::Result<(Vec<char>, HashMap<String, Node>)>{
    let input = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = input.lines().collect();
    let instructions = lines[0].chars().collect();
    let mut network = HashMap::new();
    for line in lines[2..].iter(){
        let node = Node{left: String::from(&line[7..10]), right: String::from(&line[12..15])};
        network.insert(String::from(&line[..3]), node);
    }

    Ok((instructions, network))
}

fn next_node(current: &str, instruction: char, network : &HashMap<String, Node>) -> String{
        let paths = network.get(current).unwrap();
        if instruction == 'R' {
            return String::from(&paths.right);
        } else {
            return String::from(&paths.left);
        }

}

fn part_1(instructions: &Vec<char>, network: &HashMap<String, Node>) -> usize {
    let mut current = String::from("AAA");
    let mut steps = 0;
    loop{
        if current == "ZZZ" {
            break;
        }
        let instruction = instructions[steps % instructions.len()];
        current = next_node(&current, instruction, &network);
        steps += 1;
    }

    steps
}

fn first_end(instructions: &Vec<char>, start: &str, network: &HashMap<String, Node>) -> usize {
    let mut current = String::from(start);
    let mut steps = 0;
    loop{
        if current.chars().collect::<Vec<char>>()[2] == 'Z' {
            break;
        }
        let instruction = instructions[steps % instructions.len()];
        current = next_node(&current, instruction, &network);
        steps += 1;
    }

    steps
}


fn part_2(instructions: &Vec<char>, network: &HashMap<String, Node>) -> usize {
    let starts: Vec<String> = network.keys()
                    .filter(|s| s.chars().collect::<Vec<char>>()[2] == 'A')
                    .map(|s| String::from(s))
                    .collect();
    let ends = starts.iter().map(|x| first_end(instructions, x, network));
    
    ends.reduce(|acc, x| acc.lcm(&x)).unwrap()

}

fn main() -> std::io::Result<()>{
    let (instructions, network) = get_input("input.txt")?;
    println!("Part 1: {}", part_1(&instructions, &network));
    println!("Part 2: {}", part_2(&instructions, &network));

    Ok(())
}
