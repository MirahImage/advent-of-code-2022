use line_parser::read_lines;

fn main() {
    let elves = day1::lines_to_elves(read_lines("/Users/mgary/workspace/advent-of-code-2022/day1/input").unwrap());
    let mut calories_list: Vec<i32> = elves.iter().map(|elf| { day1::total_calories(elf) }).collect();
    calories_list.sort();
    println!{"The elf with the most calories has {} calories.", calories_list[calories_list.len()-1]}
    println!{"The top three elves are carrying {} calories.", calories_list.iter().rev().take(3).sum::<i32>()}
}
