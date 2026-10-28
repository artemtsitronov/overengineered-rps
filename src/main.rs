use rand::{Rng, seq::IndexedRandom};
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    const ALL: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    pub fn vs(self, other: Move) -> Outcome {
        match (self, other) {
            (a, b) if a == b => Outcome::Draw,
            (a, b) if a.beats(b) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Move::Rock => "Rock",
            Move::Paper => "Paper",
            Move::Scissors => "Scissors",
        };
        write!(f, "{name}")
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "r" | "rock" => Ok(Move::Rock),
            "p" | "paper" => Ok(Move::Paper),
            "s" | "scissors" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Outcome::Win => "You won!",
            Outcome::Lose => "You lost!",
            Outcome::Draw => "It's a draw!",
        };
        write!(f, "{name}")
    }
}

fn choose(rng: &mut impl Rng, previous_answers: &mut Vec<String>) -> Move {
    *Move::ALL.choose(rng).unwrap()
}

fn main() {
    let mut rng = rand::rng();
    let mut previous_answers: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        let robot = choose(&mut rng, &mut previous_answers);

        print!("\nChoose Rock, Paper, Scissors, or Exit: ");
        io::stdout().flush().unwrap();

        let bytes = io::stdin()
            .read_line(&mut input)
            .expect("Oops, rps input failed :(");
        if bytes == 0 {
            break;
        }

        let choice = input.trim();
        if choice.eq_ignore_ascii_case("exit") {
            break;
        }

        let Ok(hooman) = choice.parse::<Move>() else {
            println!("Try Rock, Paper or Scissors");
            continue;
        };

        let outcome = hooman.vs(robot);
        println!(" You chose {hooman}, the robot chose {robot}.");
        println!(" {outcome}");
    }
}
