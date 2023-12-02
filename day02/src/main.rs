use std::ops::Add;

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, Default)]
struct Game {
    red: u64,
    green: u64,
    blue: u64,
}

impl Add<Game> for Game {
    type Output = Game;

    fn add(self, rhs: Game) -> Self::Output {
        Game {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Game {
    fn max(self, rhs: Game) -> Game {
        Game {
            red: core::cmp::max(self.red, rhs.red),
            green: core::cmp::max(self.green, rhs.green),
            blue: core::cmp::max(self.blue, rhs.blue),
        }
    }

    fn valid_for(self, bounds: Game) -> bool {
        self.red <= bounds.red &&
            self.green <= bounds.green &&
            self.blue <= bounds.blue
    }

    fn power(self) -> u64 {
        self.red * self.blue * self.green
    }
}

fn main() {
    let game_maxes = INPUT.lines().map(|mut line| {
        line = &line[line.find(": ").unwrap() + 2..];
        line.split("; ").map(|round| {
            round.split(", ").map(|entry| {
                let space_idx = entry.find(" ").unwrap();
                let count = entry[..space_idx].parse().unwrap();
                match &entry[space_idx + 1..] {
                    "red" => Game { red: count, green: 0, blue: 0 },
                    "green" => Game { red: 0, green: count, blue: 0 },
                    "blue" => Game { red: 0, green: 0, blue: count },
                    invalid => panic!("invalid color: {invalid}"),
                }
            }).reduce(Game::add).unwrap()
        }).reduce(Game::max).unwrap()
    }).collect::<Vec<Game>>();

    let p1 = game_maxes.iter().copied().enumerate().map(|(i, max)| {
        if max.valid_for(Game { red: 12, green: 13, blue: 14 }) {
            i + 1
        } else {
            0
        }
    }).sum::<usize>();

    let p2 = game_maxes.iter().copied().map(Game::power).sum::<u64>();

    println!("p1: {p1}, p2: {p2}");
}
