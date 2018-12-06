use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut doubles: u32 = 0;
    let mut triples: u32 = 0;

    for line in input.lines() {
        let mut counters: HashMap<char, u32> = HashMap::new();

        for c in line.chars() {
            *counters.entry(c).or_insert(0) += 1;
        }

        let unique_tallies: HashSet<&u32> = counters.values().collect();
        if unique_tallies.contains(&2) {
            doubles += 1;
        }
        if unique_tallies.contains(&3) {
            triples += 1;
        }
    }
    doubles * triples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process(
                &vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]
                    .join("\n")
            ).to_string(),
            "12"
        );
    }
}
