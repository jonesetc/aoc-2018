use std::collections::HashMap;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

struct Item {
    value: u32,
    prev: usize,
    next: usize,
}

struct Ring {
    items: Vec<Item>,
}

impl Ring {
    fn new() -> Self {
        Self {
            items: vec![
                Item {
                    value: 0,
                    prev: 0,
                    next: 0,
                }
            ]
        }
    }

    fn remove(&mut self, i: usize) -> u32 {
        println!("adding value {}", i);

        let (prev, next) = (self.items[i].prev, self.items[i].next);

        self.items[prev].next = next;
        self.items[next].prev = prev;

        self.items[i].value.clone()
    }

    fn insert_after(&mut self, i: usize, val: u32) {
        let index = self.items.len();
        self.items.push(Item {
            value: val,
            prev: i,
            next: self.items[i].next,
        });
        
        self.items[i].next = index;
    }

    fn backward(&self, i: usize, count: usize) -> usize {
        let mut curr = i;
        for _ in 0..count {
            curr = self.items[curr].prev;
        }
        println!("reversing to {}", curr);
        curr
    }

    fn forward(&self, i: usize, count: usize) -> usize {
        let mut curr = i;
        for _ in 0..count {
            curr = self.items[curr].next;
        }
        println!("advancing to {}", curr);
        curr
    }
}

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
    let mut current_marble: usize = 0;
    let mut circle = Ring::new();

    for (player, marble) in { 0..players }.cycle().skip(1).zip({ 1..=last_marble }) {
        if marble % 23 == 0 {
            let score = scores.entry(player).or_insert(0);
            *score += marble;

            current_marble = circle.backward(current_marble, 6);
            let remove = circle.backward(current_marble, 1);
            *score += circle.remove(remove);
        } else {
            current_marble = circle.forward(current_marble, 1);
            circle.insert_after(current_marble, marble);
            current_marble = circle.forward(current_marble, 1);
        }
    }

    scores.values().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example0() {
        assert_eq!(
            process("9 players; last marble is worth 25 points").to_string(),
            "32"
        );
    }

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
