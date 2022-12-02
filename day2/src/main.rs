use line_parser::read_lines;

fn main() {
    let moves = read_lines("/Users/mgary/workspace/advent-of-code-2022/day2/input").unwrap();
    let sum_given_moves: i32 = moves.iter().map(|line| day2::score_given_moves(line).unwrap()).sum::<i32>();
    let sum_given_outcomes: i32 = moves.iter().map(|line| day2::score_given_outcome(line).unwrap()).sum::<i32>();
    println!("Total score given moves: {}", sum_given_moves);
    println!("Total score given outcomes: {}", sum_given_outcomes);
}
