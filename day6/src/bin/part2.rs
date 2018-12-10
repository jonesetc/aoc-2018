use std::cmp::{max, min};
use std::num::ParseIntError;
use std::u32;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn parse_coords(s: &str) -> Result<(u32, u32), ParseIntError> {
    let parts: Vec<&str> = s.split(", ").collect();
    Ok((parts[0].parse::<u32>()?, parts[1].parse::<u32>()?))
}

fn main() {
    println!("answer is: {}", process(INPUT, 10_000).to_string())
}

fn process(input: &str, cutoff: u32) -> impl ToString {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (u32::MAX, u32::MIN, u32::MAX, u32::MIN);

    let coords: Vec<(u32, u32)> = input
        .lines()
        .map(|line| parse_coords(line).unwrap())
        .map(|(x, y)| {
            if x < min_x {
                min_x = x
            }
            if x > max_x {
                max_x = x
            }
            if y < min_y {
                min_y = y
            }
            if y > max_y {
                max_y = y
            }
            (x, y)
        })
        .collect::<Vec<(u32, u32)>>();

    { min_x..=max_x }
        .map(|x| {
            { min_y..=max_y }
                .map(|y| {
                    coords
                        .iter()
                        .map(|coord| {
                            (max(coord.0, x) - min(coord.0, x))
                                + (max(coord.1, y) - min(coord.1, y))
                        })
                        .sum::<u32>()
                })
                .filter(|&distances| distances < cutoff)
                .count() as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process(
                &vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9",].join("\n"),
                32
            )
            .to_string(),
            "16"
        );
    }
}
