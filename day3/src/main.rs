use std::fs;

struct Rugsack{
    first: Vec<Item>,
    second: Vec<Item>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Item {
    character: char,
    prio: u32,
}

impl Rugsack {
    fn contains(&self, item: &Item) -> bool {
        if self.first.contains(&item) || self.second.contains(&item){
            return true;
        }
        return false;
    }
    fn find_duplicate(&self) -> Item {
        for i in self.first.clone(){
            if self.second.contains(&i) {
                return i;
            }
        }
        panic!("Found no duplicates");
    }
}

fn build_rugsack(content: String) -> Rugsack {
    let mut rs = Rugsack{ first: Vec::new(), second: Vec::new()};
    for (i, char) in content.chars().enumerate(){
        let item = build_item(char);
        if content.len()/2 > i {
            rs.first.push(item);
        } else {
            rs.second.push(item);
        }
    }
    return rs;
}

fn build_item(item_code: char) -> Item {
    let num: u32;
    if item_code.is_lowercase() {
        num = item_code as u32 - 96;
    } else {
        num = item_code as u32 - 38;
    }
    return Item{ character: item_code, prio: num }
}

fn find_common_item(group: Vec<Rugsack>) -> Item{
    if group.len() != 3 {
        panic!("Invalid group length");
    }
    for item in &group[0].first {
        if group[1].contains(item) && group[2].contains(item) {
            return build_item(item.character);
        }
    }
    for item in &group[0].second {
        if group[1].contains(item) && group[2].contains(item) {
            return build_item(item.character);
        }
    }
    panic!("Did not find a common item");
}

fn main() {
    let data = fs::read_to_string("input")
        .expect("Input file to be created and readable");
    let mut duplicate_sum = 0;
    let mut badge_sum = 0;
    let mut group: Vec<Rugsack> = Vec::new();
    for (j, line) in data.lines().enumerate(){
        let rs = build_rugsack(line.to_string());
        let duplicate = rs.find_duplicate();
        duplicate_sum += duplicate.prio;
        group.push(rs);
        if (j + 1) % 3 == 0 {
            let common_item = find_common_item(group);
            badge_sum += common_item.prio;
            group = Vec::new();
        }
    }
    println!("Duplicate item sum: {}", duplicate_sum);
    println!("Badge sum: {}", badge_sum);
}
