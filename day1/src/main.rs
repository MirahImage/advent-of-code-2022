use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").unwrap();
    let elves = day1::lines_to_elves(lines);
    let mut calories_list: Vec<i32> = elves.iter().map(|elf| { day1::total_calories(elf) }).collect();
    calories_list.sort();
    println!{"The elf with the most calories has {} calories.", calories_list[calories_list.len() - 1]}
}

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    io::BufReader::new(File::open(filename)?).lines().collect()
}
