use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

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
        Ok(Claim {
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
            .flat_map(|x| {
                { self.top_margin..self.top_margin + self.height }
                    .map(move |y| (x.clone(), y.clone()))
            }).collect()
    }
}

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut coords: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();

    for claim in input.lines().flat_map(|line| line.parse::<Claim>()) {
        for coord in claim.coords() {
            (*coords.entry(coord).or_insert(HashSet::new())).insert(claim.id);
        }
    }

    let mut single_ids: HashSet<u32> = HashSet::new();
    let mut multiple_ids: HashSet<u32> = HashSet::new();

    for ids in coords.values() {
        if ids.len() == 1 {
            single_ids.extend(ids);
        } else {
            multiple_ids.extend(ids);
        }
    }
    single_ids.difference(&multiple_ids).map(|id| id.to_string()).collect::<Vec<String>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            process(&vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"].join("\n"))
                .to_string(),
            "3"
        );
    }
}
