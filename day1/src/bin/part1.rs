const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process(&"+1, +1, +1".replace(", ", "\n")).to_string(), "3");
    }

    #[test]
    fn example2() {
        assert_eq!(process(&"+1, +1, -2".replace(", ", "\n")).to_string(), "0");
    }

    #[test]
    fn example3() {
        assert_eq!(process(&"-1, -2, -3".replace(", ", "\n")).to_string(), "-6");
    }
}
