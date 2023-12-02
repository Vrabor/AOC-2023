use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

#[derive(Debug, Clone)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_set(set_str: &str) -> GameSet {
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    let red_caps = re_red.captures(set_str);
    let green_caps = re_green.captures(set_str);
    let blue_caps = re_blue.captures(set_str);
    let colours = [red_caps, green_caps, blue_caps].map(|x| {
        x.map_or(0, |c| {
            c[1].parse::<usize>()
                .expect("Couldn't parse number for color")
        })
    });
    GameSet {
        red: colours[0],
        green: colours[1],
        blue: colours[2],
    }
}

fn get_input(path: &str) -> std::io::Result<Vec<Game>> {
    let input = std::fs::read_to_string(path)?;
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let col = line.find(|s| s == ':').unwrap();
        let game_num = &line[5..col]
            .parse::<usize>()
            .expect("Couldn't parse game number");
        let mut new_game = Game {
            id: *game_num,
            sets: Vec::new(),
        };
        let semis = line.match_indices(';').map(|(i, _)| i);
        let mut i = col;
        for semi in semis {
            let set_str = &line[i..semi];
            new_game.sets.push(parse_set(set_str));
            i = semi;
        }
        // handle last game, since no ;
        new_game.sets.push(parse_set(&line[i..]));
        games.push(new_game);
    }

    Ok(games)
}

fn game_valid(game: &Game) -> bool {
    game.sets
        .iter()
        .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
}

fn part_1(games: &Vec<Game>) -> usize {
    games.iter().fold(
        0,
        |acc, game| if game_valid(game) { acc + game.id } else { acc },
    )
}

fn game_power(game: &Game) -> usize {
    let mut minimum = game.sets[0].clone();
    for set in &game.sets {
        minimum.red = max(set.red, minimum.red);
        minimum.green = max(set.green, minimum.green);
        minimum.blue = max(set.blue, minimum.blue);
    }
    minimum.red * minimum.blue * minimum.green
}

fn part_2(games: &Vec<Game>) -> usize {
    games.iter().fold(0, |acc, game| acc + game_power(game))
}

fn main() -> std::io::Result<()> {
    let input = get_input("input.txt")?;
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
