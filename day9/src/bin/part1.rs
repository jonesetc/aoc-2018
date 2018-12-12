use std::collections::HashMap;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn parse(s: &str) -> Result<(u32, u32), ParseIntError> {
    let parts: Vec<u32> = s
        .split(|c: char| !c.is_numeric())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;
    Ok((parts[0], parts[1]))
}

fn process(input: &str) -> impl ToString {
    let (players, last_marble) = parse(input.trim()).unwrap();

    let mut scores: HashMap<u32, u32> = HashMap::new();
    let mut current_marble: usize = 1;
    let mut circle: Vec<u32> = vec![0, 1];

    for (player, marble) in { 0..players }.cycle().skip(2).zip({ 2..=last_marble }) {
        if marble % 23 == 0 {
            let score = scores.entry(player).or_insert(0);
            *score += marble;

            let index = (current_marble + circle.len() - 7) % circle.len();
            let adding = circle.remove(index);
            println!("adding value {}", adding);
            *score += adding;
            current_marble = index % circle.len();
        } else {
            current_marble = (current_marble + 2) % circle.len();
            circle.insert(current_marble, marble);
        }
    }

    scores.values().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process("10 players; last marble is worth 1618 points").to_string(),
            "8317"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            process("13 players; last marble is worth 7999 points").to_string(),
            "146373"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            process("17 players; last marble is worth 1104 points").to_string(),
            "2764"
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            process("21 players; last marble is worth 6111 points").to_string(),
            "54718"
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            process("30 players; last marble is worth 5807 points").to_string(),
            "37305"
        );
    }
}
