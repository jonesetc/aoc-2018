use std::collections::HashMap;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut polymers: HashMap<char, Vec<char>> = HashMap::new();

    for curr in input.trim().chars() {
        for unit in "abcdefghijklmnopqrstuvwxyz".chars() {
            if curr.to_ascii_lowercase() == unit {
                continue;
            }

            let polymer = polymers.entry(unit).or_insert(Vec::new());
            match polymer.pop() {
                Some(prev) => {
                    if prev.to_ascii_lowercase() != curr.to_ascii_lowercase() || prev == curr {
                        polymer.push(prev);
                        polymer.push(curr);
                    }
                }
                None => polymer.push(curr),
            };
        }
    }

    polymers
        .values()
        .map(|polymer| polymer.len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process("dabAcCaCBAcCcaDA").to_string(), "4");
    }
}
