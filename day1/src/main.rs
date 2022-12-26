use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input")
        .expect("File 'input' should be present and readable");
    let mut elf_list: Vec<i32> = Vec::new();
    let mut calories: i32 = 0;
    for line in data.lines() {
        if line.trim() == "" {
            elf_list.push(calories);
            calories = 0;
            continue;
        }
        calories += line.parse::<i32>().unwrap();
    }
    let max_value = elf_list.iter().max().unwrap();
    println!("The elf with the most calories have {}", max_value);
    let mut top_three: i32 = 0;
    elf_list.sort();
    for i in 1..4 {
        top_three += elf_list[elf_list.len() - i];
    }
    println!("The top three elfs have {}", top_three);
}
