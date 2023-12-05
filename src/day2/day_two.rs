use std::fs::File;
use std::{env, io};
use std::cmp::max;
use std::io::Read;
use std::str::FromStr;

const INPUT_PATH: &str = "data/input.txt";

fn read_file(path: &str) -> io::Result<String> {
    let path = env::current_dir()?.join("src/day2").join(path);
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    Ok(content)
}

fn array_from_string(data: &String) -> Vec<&str> {
    return data.lines().collect();
}
fn get_sum_of_valid_game_id(games: &Vec<Game>) -> u32 {
    games.iter().filter_map(|s| {
        if s.is_valid() {
            Some(s.game_id)
        } else {
            None
        }
    }).reduce(|acc, e| acc + e).unwrap()
}

fn get_power_of_set_of_cubes(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| game.get_power()).sum()
}

pub fn main() -> Result<DayTwoResult, io::Error> {
    let data = read_file(INPUT_PATH)?;
    let lines = array_from_string(&data);

    let mut games: Vec<Game> = vec![];

    for line in lines {
        let colon_split: Vec<&str> = line.split(":").collect();

        let game_id= colon_split[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap_or_default();

        // Alternate way to get the game id
        let game_id = line.trim_start_matches("Game ")
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or_default();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let draw_sets = colon_split[1].trim().split(";");

        for set in draw_sets {
            let draw_items = set.trim().split(", ");
            for draw_item in draw_items {
                let items: Vec<&str> = draw_item.split_whitespace().collect();

                if let Ok(color) = items[1].parse::<Color>() {
                    let count = items[0].parse::<u32>().unwrap_or_default();
                    match color {
                        Color::RED => red = max(red, count),
                        Color::GREEN => blue = max(blue, count),
                        Color::BLUE => green = max(green, count),
                    }
                }
            }
        }

        let game = Game {
            game_id,
            red,
            green,
            blue,
        };

        games.push(game);
    }

    let result = DayTwoResult {
        power_of_set_of_cubes: get_power_of_set_of_cubes(&games),
        sum_of_valid_game_id: get_sum_of_valid_game_id(&games),
    };

    Ok(result)
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct DayTwoResult {
    power_of_set_of_cubes: u32,
    sum_of_valid_game_id: u32
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    const RED: u32 = 12;
    const GREEN : u32= 13;
    const BLUE: u32 = 14;

    fn new() -> Self {
        Game {
            game_id: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_valid(&self) -> bool {
        return self.red <= Game::RED && self.green <= Game::GREEN && self.blue <= Game::BLUE;
    }

    fn get_power(&self) -> u32 {
        return self.red * self.green * self.blue
    }
}

