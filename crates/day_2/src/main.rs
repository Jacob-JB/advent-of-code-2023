
const INPUT: &str = include_str!("input.txt");

struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}

struct Round {
    red_cubes: u32,
    blue_cubes: u32,
    green_cubes: u32,
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(": ");
        let (Some(game_number), Some(rounds), None) = (parts.next(), parts.next(), parts.next())  else {
            return Err(());
        };

        let game_number = game_number.split(' ').nth(1).ok_or(())?.parse().ok().ok_or(())?;

        let rounds = rounds.split("; ")
        .map(|round| round.try_into().ok())
        .collect::<Option<Vec<Round>>>()
        .ok_or(())?;

        Ok(Game {
            game_number,
            rounds,
        })
    }
}

impl TryFrom<&str> for Round {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut round = Round {
            red_cubes: 0,
            blue_cubes: 0,
            green_cubes: 0,
        };

        for sample in value.split(", ") {
            let mut sample = sample.split(' ');

            if let (Some(cube_count), Some(cube_type)) = (sample.next(), sample.next()) {

                let cube_count: u32 = cube_count.parse().ok().ok_or(())?;

                match cube_type {
                    "red" => round.red_cubes += cube_count,
                    "blue" => round.blue_cubes += cube_count,
                    "green" => round.green_cubes += cube_count,
                    _ => return Err(()),
                }
            }
        }

        Ok(round)
    }
}

impl Round {
    fn is_possible(&self) -> bool {
        self.red_cubes <= 12 &&
        self.green_cubes <= 13 &&
        self.blue_cubes <= 14
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| round.is_possible())
    }

    fn minimum_power(&self) -> u32 {
        self.rounds.iter().map(|round| round.red_cubes).max().unwrap_or(0) *
        self.rounds.iter().map(|round| round.blue_cubes).max().unwrap_or(0) *
        self.rounds.iter().map(|round| round.green_cubes).max().unwrap_or(0)
    }
}

fn main() {
    let mut possible_games_sum = 0;
    let mut power_sum = 0;

    for line in INPUT.split("\n") {
        let game: Game = line.trim().try_into().unwrap();

        if game.is_possible() {
            println!("game {} is possible", game.game_number);
            possible_games_sum += game.game_number;
        }

        power_sum += game.minimum_power();
    }

    println!("sum of possible games {}", possible_games_sum);
    println!("sum of game powers {}", power_sum);
}
