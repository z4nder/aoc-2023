/*
The Elf put random number of cubes in the bag with
red, blue and gree colors, then show me cube and put in the bag

Im play several games and record info about each games

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue

One game has many sets separeted by ;

The Elf would know witch games are possible with that bag contains

12 red,
13 gree,
14 blue

- Read input of games
- Check the game is possible with this bag contains uppder infos
- Log the sum of ID of possible games

*/

use std::{
    collections::HashMap,
    fs::{self},
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColors {
    RED,
    GREEN,
    BLUE,
}

#[derive(Default, Debug, PartialEq)]
struct Game {
    id: i32,
    records: Vec<GameRecord>,
}

#[derive(Default, Debug, PartialEq)]
struct GameRecord {
    value: HashMap<CubeColors, i32>,
}

impl Game {
    fn create(id: i32, records: Vec<GameRecord>) -> Self {
        Self { id, records }
    }
    // fn sum_valid_ids(games: Vec<Self>, cubes_quantity: HashMap<&str, i32>) -> i32 {
    //     todo!()
    // }
}

impl GameRecord {
    fn create(red: i32, green: i32, blue: i32) -> Self {
        let mut value = HashMap::new();

        value.insert(CubeColors::RED, red);
        value.insert(CubeColors::GREEN, green);
        value.insert(CubeColors::BLUE, blue);

        GameRecord { value }
    }

    fn create_from_hash_map(value: HashMap<CubeColors, i32>) -> GameRecord {
        GameRecord { value }
    }

    fn create_records(records: Vec<&str>) -> Vec<GameRecord> {
        let mut game_records: Vec<GameRecord> = Vec::new();

        for record in records {
            let rec = Self::create(
                Self::find_cube_count_by_color(record, "red"),
                Self::find_cube_count_by_color(record, "green"),
                Self::find_cube_count_by_color(record, "blue"),
            );

            game_records.push(rec);
        }

        game_records
    }

    fn find_cube_count_by_color(record: &str, color: &str) -> i32 {
        let records: Vec<&str> = record.split(",").collect();

        for record in records {
            if record.contains(color) {
                return record
                    .replace(color, "")
                    .trim()
                    .parse()
                    .expect("Error at parse cube count");
            }
        }

        return 0;
    }
}

fn main() {
    let file = read_file("./inputs/input.txt");
    let games = parse_games(file);

    // let mut cubes_quantity = HashMap::new();

    // cubes_quantity.insert(CubeColors::RED, 12);
    // cubes_quantity.insert(CubeColors::GREEN, 13);
    // cubes_quantity.insert(CubeColors::BLUE, 14);

    // println!("Result: {:?}", sum_valid_ids(games, cubes_quantity));

    println!("Result: {:?}", get_power_of_games(games));
}

fn get_power_of_games(games: Vec<Game>) -> i32 {
    let mut tota_power = 0;

    for game in games {
        let game_record = get_min_count_color_game_record(game);
        let game_power = get_power_of_game_record(game_record);
        tota_power += game_power;
    }

    tota_power
}

fn get_power_of_game_record(game_record: GameRecord) -> i32 {
    game_record.value.get(&CubeColors::RED).unwrap()
        * game_record.value.get(&CubeColors::GREEN).unwrap()
        * game_record.value.get(&CubeColors::BLUE).unwrap()
}

fn get_min_count_color_game_record(game: Game) -> GameRecord {
    let mut max_cube_value = HashMap::new();

    max_cube_value.insert(CubeColors::RED, 0);
    max_cube_value.insert(CubeColors::GREEN, 0);
    max_cube_value.insert(CubeColors::BLUE, 0);

    for game_record in game.records {
        if game_record.value.get(&CubeColors::RED) > max_cube_value.get(&CubeColors::RED) {
            max_cube_value.insert(
                CubeColors::RED,
                game_record.value.get(&CubeColors::RED).unwrap().clone(),
            );
        }
        if game_record.value.get(&CubeColors::GREEN) > max_cube_value.get(&CubeColors::GREEN) {
            max_cube_value.insert(
                CubeColors::GREEN,
                game_record.value.get(&CubeColors::GREEN).unwrap().clone(),
            );
        }
        if game_record.value.get(&CubeColors::BLUE) > max_cube_value.get(&CubeColors::BLUE) {
            max_cube_value.insert(
                CubeColors::BLUE,
                game_record.value.get(&CubeColors::BLUE).unwrap().clone(),
            );
        }
    }

    GameRecord::create_from_hash_map(max_cube_value)
}

fn sum_valid_ids(games: Vec<Game>, cubes_quantity: HashMap<CubeColors, i32>) -> i32 {
    let mut sum = 0;

    for game in games {
        if valid_records(game.records, &cubes_quantity) {
            sum += game.id;
        }
    }

    sum
}

fn valid_records(records: Vec<GameRecord>, cubes_quantity: &HashMap<CubeColors, i32>) -> bool {
    for game_record in records {
        if game_record.value.get(&CubeColors::RED) > cubes_quantity.get(&CubeColors::RED) {
            return false;
        }
        if game_record.value.get(&CubeColors::GREEN) > cubes_quantity.get(&CubeColors::GREEN) {
            return false;
        }
        if game_record.value.get(&CubeColors::BLUE) > cubes_quantity.get(&CubeColors::BLUE) {
            return false;
        }
    }

    return true;
}

fn parse_games(file: String) -> Vec<Game> {
    let lines = file.lines();

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let game_record_string: Vec<&str> = line.split(":").collect();

        let game_id = parse_game_id(game_record_string[0]);
        let game_records = parse_game_record(game_record_string[1]);

        let records = GameRecord::create_records(game_records);

        games.push(Game::create(game_id, records));
    }

    games
}

fn parse_game_record(game_line: &str) -> Vec<&str> {
    return game_line.trim().split(";").collect();
}

fn parse_game_id(game_line: &str) -> i32 {
    return game_line
        .replace("Game", "")
        .trim()
        .parse()
        .expect("Error at parse id to i32");
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[cfg(test)]
mod tests {
    use crate::{
        get_min_count_color_game_record, get_power_of_games, parse_games, read_file, Game,
        GameRecord,
    };

    #[test]
    fn test_parse_games() {
        let records = vec![
            GameRecord::create(4, 0, 3),
            GameRecord::create(1, 2, 6),
            GameRecord::create(0, 2, 0),
        ];

        let expected_game = Game::create(1, records);

        let file = read_file("./inputs/input_test.txt");
        let games = parse_games(file);

        assert_eq!(games[0], expected_game);
    }

    #[test]
    fn test_get_min_count_color_game_record() {
        let records = vec![
            GameRecord::create(4, 0, 3),
            GameRecord::create(1, 2, 6),
            GameRecord::create(0, 2, 0),
        ];

        let game = Game::create(1, records);

        let game_record = get_min_count_color_game_record(game);

        let expecte_game_record = GameRecord::create(4, 2, 6);

        assert_eq!(game_record, expecte_game_record);
    }

    #[test]
    fn test_get_power_of_games() {
        let records = vec![
            GameRecord::create(4, 0, 3),
            GameRecord::create(1, 2, 6),
            GameRecord::create(0, 2, 0),
        ];

        let game = Game::create(1, records);

        let power = get_power_of_games(vec![game]);

        assert_eq!(power, 48);
    }
}
