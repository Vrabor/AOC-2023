use std::collections::HashSet;

struct Card {
    winning: HashSet<usize>,
    draw: HashSet<usize>,
}

impl Card {
    fn matches(&self) -> usize {
        self.winning.intersection(&self.draw).count()
    }

    fn score(&self) -> usize {
        let winning = self.matches();
        if winning > 0 {
            (2 as usize).pow((winning - 1) as u32)
        } else {
            0
        }
    }
}

fn get_input(path: &str) -> std::io::Result<Vec<Card>> {
    let input = std::fs::read_to_string(path)?;
    let mut result = Vec::new();
    for line in input.lines() {
        let num_str = line.split(':').collect::<Vec<&str>>()[1];
        let nums: Vec<&str> = num_str.split('|').collect();
        result.push(Card {
            winning: nums[0]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<usize>().unwrap())
                .collect(),
            draw: nums[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<usize>().expect("Parsing draw"))
                .collect(),
        })
    }

    Ok(result)
}

fn part_1(cards: &Vec<Card>) -> usize {
    cards.iter().fold(0, |acc, c| acc + c.score())
}

fn part_2(cards: &Vec<Card>) -> usize {
    let mut amounts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let copies = amounts[i];
        for j in 1..=card.matches() {
            amounts[i + j] += copies;
        }
    }
    amounts.iter().sum()
}

fn main() -> std::io::Result<()> {
    let input = get_input("input.txt")?;
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
