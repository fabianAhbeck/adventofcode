use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Input file to be created and readable");
    let header = data.lines().take(8).collect::<String>();
    let initial_state = get_inital_state(&header);
    let mut stacks = initial_state.clone();
    let mut stacks_advanced = initial_state;
    print_stack(&stacks);
    println!();
    for line in data.lines().skip(10) {
        let words: Vec<&str> = line.split(' ').collect();
        let number: i32 = words[1].parse().unwrap();
        let from: i32 = words[3].parse().unwrap();
        let to: i32 = words[5].parse().unwrap();
        move_boxes(number, from, to, &mut stacks);
        move_boxes_advanced(number, from, to, &mut stacks_advanced);
    }
    println!("Level 1:");
    print_stack(&stacks);
    println!("Level 2:");
    print_stack(&stacks_advanced);
}

fn move_boxes(num_to_move: i32, from: i32, to: i32, stacks: &mut HashMap<i32, Vec<String>>) {
    println!("Moving {} boxes from {} to {}", num_to_move, from, to);
    for _ in 0..num_to_move {
        let temp = stacks.get_mut(&from).unwrap().pop().unwrap();
        stacks.get_mut(&to).unwrap().push(temp);
    }
}
fn move_boxes_advanced(
    num_to_move: i32,
    from: i32,
    to: i32,
    stacks: &mut HashMap<i32, Vec<String>>,
) {
    let mut temp: Vec<String> = Vec::new();
    for _ in 0..num_to_move {
        temp.push(stacks.get_mut(&from).unwrap().pop().unwrap());
    }
    temp.reverse();
    stacks.get_mut(&to).unwrap().append(&mut temp);
}

fn print_stack(stacks: &HashMap<i32, Vec<String>>) {
    for i in 1..10 {
        for item in stacks[&i].iter() {
            print!("{}", item);
        }
        println!();
    }
}

fn get_inital_state(header: &str) -> HashMap<i32, Vec<String>> {
    let mut initial: HashMap<i32, Vec<String>> = HashMap::new();
    for i in 1..10 {
        let stack: Vec<String> = Vec::new();
        initial.insert(i, stack);
    }
    for line in (0..8).rev() {
        for column in 1..10_i32 {
            // Length of each box is 4 so offset for each column
            let mut offset: usize = (column - 1) as usize * 4;
            // Skip number of chars per line for each line
            offset += line * 35;
            let section: String = header.chars().skip(offset + 1).take(1).collect::<String>();
            if section.trim() != "" {
                initial.get_mut(&column).unwrap().push(section);
            }
        }
    }
    initial
}
