#[derive(Eq, PartialEq, Debug)]
pub struct Elf {
    calories: Vec<i32>,
}

pub fn lines_to_elves(lines: Vec<String>) -> Vec<Elf> {
    let mut elves: Vec<Elf> = lines.iter().fold(vec![Elf{calories: Vec::new()}], |mut elves, line| {
        if let Ok(cals) = line.trim().parse::<i32>() {
            elves.last_mut().unwrap().calories.push(cals)
        } else {
            elves.push(Elf{calories: Vec::new()})
        }
        elves
    });
    elves.pop(); // we've appended an extra empty Elf, get rid of it
    elves
}

pub fn total_calories(elf: &Elf) -> i32 {
    return elf.calories.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_to_elves() {
        let lines: Vec<String> = ["1000 ", "2000 ", "3000 ", "", "4000 ", "", "5000 ", "6000 ", "", "7000 ", "8000 ", "9000 ", "", "10000", ""].map(String::from).to_vec();
        assert_eq!(lines_to_elves(lines), vec![
            Elf {
                calories: vec![1000, 2000, 3000]
            },
            Elf {
                calories: vec![4000]
            },
            Elf {
                calories: vec![5000, 6000]
            },
            Elf {
                calories: vec![7000, 8000, 9000]
            },
            Elf {
                calories: vec![10000]
            }
        ])
    }

    #[test]
    fn test_total_calories() {
        assert_eq!(total_calories(&Elf {calories: vec![1000, 2000, 3000]}), 6000)
    }
}
