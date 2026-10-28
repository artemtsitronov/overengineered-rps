use rand::{Rng, seq::IndexedRandom};
use std::io::{self, Write};

fn choose(rng: &mut impl Rng, previous_answers: &mut Vec<String>) -> String {
    let moves = ["Rock", "Paper", "Scissors"];

    if previous_answers.is_empty() {
        moves.choose(rng).unwrap().to_string()
    } else {
        moves.choose(rng).unwrap().to_string() // TODO
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut previous_answers: Vec<String> = Vec::new();

    loop {
        let mut human = String::new();
        let robot = choose(&mut rng, &mut previous_answers);

        print!("\nChoose Rock, Paper, Scissors, or Exit: ");
        io::stdout().flush().unwrap();

        let bytes = io::stdin()
            .read_line(&mut human)
            .expect("Oops, rps input failed :(");
        if bytes == 0 {
            break;
        }

        let choice = human.trim();
        if choice.eq_ignore_ascii_case("exit") {
            break;
        }

        println!(" You chose {}, the robot chose {}", choice, robot);
    }
}
