use std::collections;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut current_freq: i32 = 0;
    let mut previous_freqs: collections::HashSet<i32> = [current_freq].iter().cloned().collect();

    for value in input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
    {
        current_freq += value;
        if !previous_freqs.insert(current_freq) {
            break;
        }
    }

    current_freq
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example1() {
        assert_eq!(process(&"+1, -1".replace(", ", "\n")).to_string(), "0");
    }

    #[test]
    fn example2() {
        assert_eq!(
            process(&"+3, +3, +4, -2, -4".replace(", ", "\n")).to_string(),
            "10"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            process(&"-6, +3, +8, +5, -6".replace(", ", "\n")).to_string(),
            "5"
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            process(&"+7, +7, -2, -7, -4".replace(", ", "\n")).to_string(),
            "14"
        );
    }
}
