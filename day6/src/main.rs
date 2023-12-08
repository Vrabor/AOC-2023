fn get_input_p1(path: &str) -> std::io::Result<(Vec<usize>, Vec<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = input.lines().collect();
    let times = lines[0]
        .split_whitespace()
        .filter(|s| s.chars().all(char::is_numeric))
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let distances = lines[1]
        .split_whitespace()
        .filter(|s| s.chars().all(char::is_numeric))
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    Ok((times, distances))
}

fn distance(charge: usize, time: usize) -> usize {
    charge * (time - charge)
}

fn part_1(times: Vec<usize>, distances: Vec<usize>) -> usize {
    assert!(times.len() == distances.len(), "Input different size");
    let mut res = 1;
    for i in 0..times.len() {
        let mut options = 0;
        for j in 0..times[i] {
            if distance(j, times[i]) > distances[i] {
                options += 1;
            }
        }
        res *= options;
    }
    res
}
fn get_input_p2(path: &str) -> std::io::Result<(usize, usize)> {
    let input = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0]
        .chars()
        .filter(|c| char::is_numeric(*c))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = lines[1]
        .chars()
        .filter(|c| char::is_numeric(*c))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Ok((time, distance))
}

fn part_2(time: usize, distance_: usize) -> usize {
    let mut options = 0;
    for j in 0..time {
        if distance(j, time) > distance_ {
            options += 1;
        }
    }
    options
}

fn main() -> std::io::Result<()> {
    let (times, distances) = get_input_p1("input.txt")?;
    println!("Part 1: {}", part_1(times, distances));
    let (time, distance) = get_input_p2("input.txt")?;
    println!("Part 2: {}", part_2(time, distance));
    Ok(())
}
