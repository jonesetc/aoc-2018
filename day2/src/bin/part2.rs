use std::collections::HashSet;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut previous_ids: HashSet<(&str, &str)> = HashSet::new();

    for line in input.lines() {
        for i in { 0..line.len() } {
            let masked_id: (&str, &str) = (&line[..i], &line[i + 1..]);
            if !previous_ids.insert(masked_id) {
                return format!("{}{}", masked_id.0, masked_id.1);
            }
        }
    }
    panic!("something went wrong")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process(
                &vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"].join("\n")
            )
            .to_string(),
            "fgij"
        );
    }
}
