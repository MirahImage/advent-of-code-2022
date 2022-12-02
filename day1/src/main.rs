use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let elves = day1::lines_to_elves(read_lines("./input").unwrap());
    let mut calories_list: Vec<i32> = elves.iter().map(|elf| { day1::total_calories(elf) }).collect();
    calories_list.sort();
    let length=calories_list.len();
    println!{"The elf with the most calories has {} calories.", calories_list[length - 1]}
    println!{"The top three elves are carrying {} calories.", calories_list[length - 1] + calories_list[length - 2] + calories_list[length - 3]}
}

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    io::BufReader::new(File::open(filename)?).lines().collect()
}
