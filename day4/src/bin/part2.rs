use std::collections::{BTreeMap, HashMap};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../../etc/input-part2.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EntryTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl FromStr for EntryTime {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u32> = s
            .split(|c: char| !c.is_numeric())
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, Self::Err>>()?;
        Ok(Self {
            year: parts[0],
            month: parts[1],
            day: parts[2],
            hour: parts[3],
            minute: parts[4],
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum EntryAction {
    BEGIN(u32),
    SLEEP,
    WAKE,
}

impl FromStr for EntryAction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "falls asleep" {
            Ok(EntryAction::SLEEP)
        } else if s == "wakes up" {
            Ok(EntryAction::WAKE)
        } else {
            s.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
                .map(|guard_id| EntryAction::BEGIN(guard_id))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    time: EntryTime,
    action: EntryAction,
}

impl FromStr for Entry {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = s[1..17].parse::<EntryTime>()?;
        let action = s[19..].parse::<EntryAction>()?;

        Ok(Self { time, action })
    }
}

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let log: BTreeMap<EntryTime, EntryAction> = input
        .lines()
        .flat_map(|line| line.parse::<Entry>())
        .map(|entry| (entry.time, entry.action))
        .collect();

    let mut sleeping_tallies: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut curr_state: (u32, u32) = (0, 0);

    for (time, action) in log.iter() {
        match action {
            &EntryAction::BEGIN(guard) => {
                curr_state.0 = guard;
                curr_state.1 = time.minute;
            }
            EntryAction::SLEEP => {
                curr_state.1 = time.minute;
            }
            EntryAction::WAKE => {
                for i in { curr_state.1..time.minute } {
                    *sleeping_tallies
                        .entry(curr_state.0)
                        .or_insert(HashMap::new())
                        .entry(i)
                        .or_insert(0) += 1;
                }
            }
        };
    }

    let (_, guard, minute) = sleeping_tallies
        .iter()
        .map(|(guard, tallies)| {
            let (max_tally, max_minute) =
                tallies
                    .iter()
                    .fold((0, 0), |(max_tally, max_minute), (&minute, &tally)| {
                        if tally > max_tally {
                            (tally, minute)
                        } else {
                            (max_tally, max_minute)
                        }
                    });
            (max_tally, guard, max_minute)
        })
        .max()
        .unwrap();

    guard * minute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            process(
                &vec![
                    "[1518-11-01 00:00] Guard #10 begins shift",
                    "[1518-11-01 00:05] falls asleep",
                    "[1518-11-01 00:25] wakes up",
                    "[1518-11-01 00:30] falls asleep",
                    "[1518-11-01 00:55] wakes up",
                    "[1518-11-01 23:58] Guard #99 begins shift",
                    "[1518-11-02 00:40] falls asleep",
                    "[1518-11-02 00:50] wakes up",
                    "[1518-11-03 00:05] Guard #10 begins shift",
                    "[1518-11-03 00:24] falls asleep",
                    "[1518-11-03 00:29] wakes up",
                    "[1518-11-04 00:02] Guard #99 begins shift",
                    "[1518-11-04 00:36] falls asleep",
                    "[1518-11-04 00:46] wakes up",
                    "[1518-11-05 00:03] Guard #99 begins shift",
                    "[1518-11-05 00:45] falls asleep",
                    "[1518-11-05 00:55] wakes up",
                ]
                .join("\n")
            )
            .to_string(),
            "4455"
        );
    }
}
