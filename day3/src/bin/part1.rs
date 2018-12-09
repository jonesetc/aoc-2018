use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../../etc/input-part1.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Claim {
    id: u32,
    left_margin: u32,
    top_margin: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u32> = s
            .split(|c: char| !c.is_numeric())
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, Self::Err>>()?;
        Ok(Self {
            id: parts[0],
            left_margin: parts[1],
            top_margin: parts[2],
            width: parts[3],
            height: parts[4],
        })
    }
}

impl Claim {
    pub fn coords(&self) -> Vec<(u32, u32)> {
        { self.left_margin..self.left_margin + self.width }
            .flat_map(|x| { self.top_margin..self.top_margin + self.height }.map(move |y| (x, y)))
            .collect()
    }
}

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut coords: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in input.lines().flat_map(|line| line.parse::<Claim>()) {
        for coord in claim.coords() {
            *coords.entry(coord).or_insert(0) += 1;
        }
    }

    coords.values().filter(|&&tally| tally > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        assert_eq!(
            "#1 @ 1,3: 4x4".parse::<Claim>(),
            Ok(Claim {
                id: 1,
                left_margin: 1,
                top_margin: 3,
                width: 4,
                height: 4
            })
        );
    }

    #[test]
    fn area() {
        assert_eq!(
            Claim {
                id: 1,
                left_margin: 1,
                top_margin: 3,
                width: 4,
                height: 4
            }
            .coords(),
            vec![
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
            ]
        );
    }

    #[test]
    fn example() {
        assert_eq!(
            process(&vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"].join("\n"))
                .to_string(),
            "4"
        );
    }
}
