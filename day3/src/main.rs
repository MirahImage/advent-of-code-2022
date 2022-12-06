use line_parser::read_lines;

fn main() {
    let contents_list = read_lines("/Users/mgary/workspace/advent-of-code-2022/day3/input").unwrap();
    let match_priority: u32 = contents_list.iter().map(|contents| day3::to_priority(day3::find_duplicate(contents)).unwrap()).sum();
    println!("The total match priority is {}", match_priority);
    let badge_priority: u32 = contents_list.chunks(3).map(|chunk| {
        day3::to_priority(day3::find_commmon(&chunk[0], &chunk[1], &chunk[2])).unwrap()
    }).sum();
    println!("The total badge priority is {}", badge_priority);
}
