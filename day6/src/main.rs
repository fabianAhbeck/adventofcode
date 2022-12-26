use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("File need to be present and readable");
    let answer_one = find_unique_char_occurance(4, &data);
    let answer_two = find_unique_char_occurance(14, &data);
    println!("Answer one: {}\nAnswer Two: {}", answer_one, answer_two);
}

fn find_unique_char_occurance(length: i32, string: &str) -> i32 {
    let mut previous: Vec<char> = Vec::new();
    for (index, character) in string.chars().enumerate() {
        if previous.len() == length as usize - 1 {
            if previous.contains(&character) || has_duplicates(&previous) {
                previous.remove(0);
            } else {
                return index as i32 + 1;
            }
        }
        previous.push(character);
    }
    0
}

fn has_duplicates(input: &[char]) -> bool {
    for (index, char) in input.iter().enumerate() {
        for i in input.iter().skip(index + 1) {
            if i == char {
                return true;
            }
        }
    }
    false
}
