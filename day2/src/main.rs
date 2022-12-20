use std::fs;

enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice{
    fn get_value(self) -> i32{
        match self {
            Choice::Rock => return 1,
            Choice::Paper => return 2,
            Choice::Scissor => return 3,
        }
    }
    fn new(x: char) -> Choice {
        match x.to_ascii_lowercase() {
            'a' | 'x' => return Choice::Rock,
            'b' | 'y' => return Choice::Paper,
            'c' | 'z' => return Choice::Scissor,
            _ => panic!("Input is not valid!"),
        }
    }
}

fn make_choice(enemy: &Choice, outcome: char) -> Choice {
    match outcome.to_ascii_lowercase() {
        'x' => {
            match enemy {
                Choice::Rock => return Choice::Scissor,
                Choice::Paper => return Choice::Rock,
                Choice::Scissor => return Choice::Paper,
            }
        }
        'y' => {
            match enemy {
                Choice::Rock => return Choice::Rock,
                Choice::Paper => return Choice::Paper,
                Choice::Scissor => return Choice::Scissor,
            }
        }
        'z' => {
            match enemy {
                Choice::Rock => return Choice::Paper,
                Choice::Paper => return Choice::Scissor,
                Choice::Scissor => return Choice::Rock,
            }
        }
        _ => panic!("Invalid input"),
    }

}

fn get_row_result(line: String, pick_choice: bool) -> i32 {
    let mut itter = line.chars();
    let their = Choice::new(itter.next().expect("Row does not contain any chars"));
    let _ = itter.next();
    let our:Choice;
    if pick_choice {
        our = make_choice(&their, itter.next().expect("Row does not contain any chars"));
    } else {
        our = Choice::new(itter.next().expect("Row does not contain any chars"));
    }
    return match_result(&our, &their) + our.get_value();
}

fn match_result(player: &Choice, enemy: &Choice) -> i32{
    match player {
        Choice::Rock => {
            match enemy{
                Choice::Rock => return 3,
                Choice::Paper => return 0,
                Choice::Scissor => return 6,
            }
        }
        Choice::Paper => {
            match enemy{
                Choice::Rock => return 6,
                Choice::Paper => return 3,
                Choice::Scissor => return 0,
            }
        }
        Choice::Scissor => {
            match enemy{
                Choice::Rock => return 0,
                Choice::Paper => return 6,
                Choice::Scissor => return 3,
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string("input")
        .expect("Input file to be created and readable");
    let mut total: i32 = 0;
    let mut total_round_2: i32 = 0;
    for line in data.lines(){
        total += get_row_result(line.to_string(), false);
        total_round_2 += get_row_result(line.to_string(), true);
    }
    println!("{} {}", total, total_round_2)
}

