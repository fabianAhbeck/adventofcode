use std::fs;
fn main() {
    let data = fs::read_to_string("input")
        .expect("Input file to be created and readable");

    // Variables to keep track of the overlapping ranges
    let mut overlapping_full = 0;
    let mut overlapping_some = 0;

    for line in data.lines() {
        // Split the line to one part per elf
        let ranges: Vec<&str> = line.split(',').collect();
        if ranges.len() != 2 {
            panic!("Line badly formated");
        }
        // break out the min and max for each elf
        let mut elf1: Vec<i32> = Vec::new();
        for i in ranges[0].split('-') {
            elf1.push(i.parse::<i32>().unwrap());
        }
        let mut elf2: Vec<i32> = Vec::new();
        for i in ranges[1].split('-') {
            elf2.push(i.parse::<i32>().unwrap());
        }
        if elf1.len() != 2 || elf2.len() != 2 {
            panic!("Line badly formated");
        }
        // Test for a full overlap
        if (elf1[0] <= elf2[0] && elf1[1] >= elf2[1]) || 
            (elf2[0] <= elf1[0] && elf2[1] >= elf1[1])
        {
            overlapping_full += 1;
            overlapping_some += 1;
        // Test for any overlap
        } else if !((elf1[0] < elf2[0] && elf1[1] < elf2[0])
            || (elf1[0] > elf2[1] && elf1[1] > elf2[1]))
        {
            overlapping_some += 1;
        }
    }
    // Print result
    println!("Overlapping fully: {}", overlapping_full);
    println!("Overlapping some: {}", overlapping_some);
}
