use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::num::ParseIntError;
use std::u32;

const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn parse_coords(s: &str) -> Result<(u32, u32), ParseIntError> {
    let parts: Vec<&str> = s.split(", ").collect();
    Ok((parts[0].parse::<u32>()?, parts[1].parse::<u32>()?))
}

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
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

    let grid: BTreeMap<u32, BTreeMap<u32, u32>> = { min_x..=max_x }
        .map(|x| {
            (
                x,
                { min_y..=max_y }
                    .flat_map(|y| {
                        let mut distances: Vec<(u32, u32)> = coords
                            .iter()
                            .enumerate()
                            .map(|(i, coord)| {
                                (
                                    (max(coord.0, x) - min(coord.0, x))
                                        + (max(coord.1, y) - min(coord.1, y)),
                                    i as u32,
                                )
                            })
                            .collect();
                        distances.sort();
                        if distances[0].0 != distances[1].0 {
                            Some((y, distances[0].1))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    let excluded_claims: BTreeSet<u32> = { min_x..=max_x }
        .flat_map(|x| grid.get(&x).unwrap().get(&min_y))
        .chain({ min_y..=max_y }.flat_map(|y| grid.get(&min_x).unwrap().get(&y)))
        .chain({ min_y..=max_y }.flat_map(|y| grid.get(&max_x).unwrap().get(&y)))
        .chain({ min_x..=max_x }.flat_map(|x| grid.get(&x).unwrap().get(&max_y)))
        .cloned()
        .collect();

    grid.iter()
        .fold(BTreeMap::new(), |mut tallies, (_, ys)| {
            ys.iter()
                .filter(|(_, claim)| !excluded_claims.contains(claim))
                .for_each(|(_, claim)| *tallies.entry(claim).or_insert(0) += 1);
            tallies
        })
        .values()
        .max()
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process(&vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9",].join("\n")).to_string(),
            "17"
        );
    }
}
