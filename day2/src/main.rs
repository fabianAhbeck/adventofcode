use ndarray::arr2;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone)]
enum Result {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

impl Choice {
    fn get_value(self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
    fn new(x: char) -> Choice {
        match x.to_ascii_lowercase() {
            'a' | 'x' => Choice::Rock,
            'b' | 'y' => Choice::Paper,
            'c' | 'z' => Choice::Scissor,
            _ => panic!("Input is not valid!"),
        }
    }
}

fn make_choice(enemy: &Choice, outcome: Result) -> Choice {
    //        Rock    Paper     Scissor
    // Loose Scissor  Rock      Paper
    // Draw   Rock    Paper     Scissor
    // Win    Paper  Scissor     Rock
    let matrix = arr2(&[
        [Choice::Scissor, Choice::Rock, Choice::Paper],
        [Choice::Rock, Choice::Paper, Choice::Scissor],
        [Choice::Paper, Choice::Scissor, Choice::Rock],
    ]);
    let row: usize = outcome as usize;
    let column: usize = (enemy.get_value() - 1) as usize;
    return *matrix
        .get((row, column))
        .expect("Result not found.")
}

fn get_row_result(line: String, given_result: bool) -> i32 {
    let mut itter = line.chars();
    let their = Choice::new(itter.next().expect("Row does not contain any chars"));
    let _ = itter.next();
    let our = itter.next().expect("Row does not contain any chars");
    let our_choice: Choice;
    if given_result {
        let result: Result = match our.to_ascii_uppercase() {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Bad format of data."),
        };
        our_choice = make_choice(&their, result);
        return result as i32 * 3 + our_choice.get_value();
    }
    our_choice = Choice::new(our);
    match_result(&our_choice, &their) + our_choice.get_value()
}

fn match_result(player: &Choice, enemy: &Choice) -> i32 {
    //   Result table based on choices where
    //   Where
    //   0 = Rock
    //   1 = Paper
    //   2 = Scissor
    //
    //   0 1 2
    // 0 3 0 6
    // 1 6 3 0
    // 2 0 6 3
    let matrix = arr2(&[[3, 0, 6], [6, 3, 0], [0, 6, 3]]);
    let row: usize = (player.get_value() - 1) as usize;
    let column: usize = (enemy.get_value() - 1) as usize;
    return *matrix
        .get((row, column))
        .expect("Result not found.");
}

fn main() {
    let data = read_to_string("input")
        .expect("Input file to be created and readable");
    let mut total: i32 = 0;
    let mut total_round_2: i32 = 0;
    for line in data.lines() {
        total += get_row_result(line.to_string(), false);
        total_round_2 += get_row_result(line.to_string(), true);
    }
    println!("{} {}", total, total_round_2)
}
