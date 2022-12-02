pub fn score_given_moves(line: &str) -> Result<i32, &'static str> {
    let round = line.split_once(' ').unwrap();
    match round {
        ("A", "X") => Ok(4),
        ("A", "Y") => Ok(8),
        ("A", "Z") => Ok(3),
        ("B", "X") => Ok(1),
        ("B", "Y") => Ok(5),
        ("B", "Z") => Ok(9),
        ("C", "X") => Ok(7),
        ("C", "Y") => Ok(2),
        ("C", "Z") => Ok(6),
        _ => Err("unknown move")
    }
}

pub fn score_given_outcome(line: &str) -> Result<i32, &'static str> {
    let round = line.split_once(' ').unwrap();
    match round {
        ("A", "X") => Ok(3),
        ("A", "Y") => Ok(4),
        ("A", "Z") => Ok(8),
        ("B", "X") => Ok(1),
        ("B", "Y") => Ok(5),
        ("B", "Z") => Ok(9),
        ("C", "X") => Ok(2),
        ("C", "Y") => Ok(6),
        ("C", "Z") => Ok(7),
        _ => Err("unknown move")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_given_moves() {
        assert_eq!(score_given_moves(&"A Y".to_string()).unwrap(), 8);
        assert_eq!(score_given_moves(&"B X".to_string()).unwrap(), 1);
        assert_eq!(score_given_moves(&"C Z".to_string()).unwrap(), 6);
    }

    #[test]
    fn test_score_given_outcome() {
        assert_eq!(score_given_outcome(&"A Y".to_string()).unwrap(), 4);
        assert_eq!(score_given_outcome(&"B X".to_string()).unwrap(), 1);
        assert_eq!(score_given_outcome(&"C Z".to_string()).unwrap(), 7);
    }
}