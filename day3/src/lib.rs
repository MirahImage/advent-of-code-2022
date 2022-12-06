pub fn find_duplicate(contents: &str) -> Option<char> {
    let (compartment1, compartment2) = contents.split_at(contents.len()/2);
    compartment1.chars().fold(None, |acc, item| {
        if compartment2.contains(item) { Some(item) } else { acc }
    })
}

pub fn to_priority(item: Option<char>) -> Option<u32> {
    match item {
        Some(c) if c.is_ascii_lowercase() => Some((c as u32) - 96),
        Some(c) if c.is_ascii_uppercase() => Some((c as u32) - 38),
        _ => None
    }
}

pub fn find_commmon(c1: &str, c2: &str, c3: &str) -> Option<char> {
    c1.chars().fold(None, |acc, item| {
        if c2.contains(item) && c3.contains(item) { Some(item) } else { acc }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
        assert_eq!(find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), Some('L'));
        assert_eq!(find_duplicate("PmmdzqPrVvPwwTWBwg"), Some('P'));
        assert_eq!(find_duplicate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), Some('v'));
        assert_eq!(find_duplicate("ttgJtRGJQctTZtZT"), Some('t'));
        assert_eq!(find_duplicate("CrZsJsPPZsGzwwsLwLmpwMDw"), Some('s'));
        assert_eq!(find_duplicate("ab"), None);
    }

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority(Some('p')), Some(16));
        assert_eq!(to_priority(Some('L')), Some(38));
        assert_eq!(to_priority(Some('P')), Some(42));
        assert_eq!(to_priority(Some('v')), Some (22));
        assert_eq!(to_priority(None), None);
    }

    #[test]
    fn test_find_common() {
        assert_eq!(find_commmon("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"), Some('r'));
        assert_eq!(find_commmon("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"), Some('Z'));
        assert_eq!(find_commmon("abc", "def", "geh"), None);
    }
}