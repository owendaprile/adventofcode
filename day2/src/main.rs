use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

impl Cube {
    fn from_str(cube: &str) -> Cube {
        match cube {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => panic!("what the hell is wrong with you?"),
        }
    }
}

#[derive(Debug)]
struct Round {
    cubes: HashMap<Cube, u64>,
}

impl Round {
    fn new(red: u64, green: u64, blue: u64) -> Round {
        let cubes = HashMap::from([(Cube::Red, red), (Cube::Green, green), (Cube::Blue, blue)]);

        Round { cubes }
    }

    fn set_count_for_cube(&mut self, cube: Cube, count: u64) {
        self.cubes.insert(cube, count);
    }

    fn get_count_for_cube(&self, cube: &Cube) -> u64 {
        self.cubes.get(&cube).unwrap_or(&0).clone()
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn new(id: usize) -> Game {
        Game {
            id: id,
            rounds: vec![],
        }
    }

    fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    fn max_seen_for_cube(&self, cube: Cube) -> u64 {
        self.rounds
            .iter()
            .map(|round| round.get_count_for_cube(&cube))
            .max()
            .unwrap()
    }
}

fn read_games_from_string(contents: String) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    for line in contents.lines() {
        let mut line = line.split(":");

        let game_id = line
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut game = Game::new(game_id);

        for round in line.next().unwrap().split(";") {
            let cubes = round.split(",");

            let mut round_struct = Round::new(0, 0, 0);

            for cube in cubes {
                let mut thing = cube.trim().split(" ");

                let count = thing.next().unwrap().parse::<u64>().unwrap();

                let color: &str = thing.next().unwrap();

                let color_struct = match color {
                    "red" => Cube::Red,
                    "green" => Cube::Green,
                    "blue" => Cube::Blue,
                    _ => panic!("that shouldn't happen"),
                };

                round_struct.set_count_for_cube(color_struct, count);
            }

            game.add_round(round_struct);
        }

        games.push(game);
    }

    games
}

fn get_possible_games(games: &Vec<Game>, red: u64, green: u64, blue: u64) -> Vec<usize> {
    let mut possible_games: Vec<usize> = vec![];

    for game in games {
        let max_red = game.max_seen_for_cube(Cube::Red);
        let max_green = game.max_seen_for_cube(Cube::Green);
        let max_blue = game.max_seen_for_cube(Cube::Blue);

        if max_red <= red && max_green <= green && max_blue <= blue {
            possible_games.push(game.id);
        }
    }

    possible_games
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();

    let games = read_games_from_string(std::fs::read_to_string(file_path).unwrap());

    let mut sum: u64 = 0;

    for game in games {
        let min_possible_red = game.max_seen_for_cube(Cube::Red);
        let min_possible_green = game.max_seen_for_cube(Cube::Green);
        let min_possible_blue = game.max_seen_for_cube(Cube::Blue);

        dbg!(min_possible_red, min_possible_green, min_possible_blue);

        let power = min_possible_red * min_possible_green * min_possible_blue;

        sum += power;
    }

    println!("The answer to part two is {}", sum);
}
