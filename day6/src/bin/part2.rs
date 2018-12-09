const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    input.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process(INPUT).to_string(), INPUT);
    }
}
