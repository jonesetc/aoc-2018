const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut polymer: Vec<char> = Vec::new();

    for curr in input.trim().chars() {
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

    polymer.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process("dabAcCaCBAcCcaDA").to_string(), "10");
    }
}
