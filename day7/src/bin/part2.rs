use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../../etc/input-part2.txt");
const ASCII_CAP_OFFSET: u32 = 64;

fn parse_deps(s: &str) -> (char, char) {
    let chars: Vec<char> = s.chars().collect();
    (chars[5], chars[36])
}

fn main() {
    println!("answer is: {}", process(INPUT, 5, 60).to_string())
}

fn process(input: &str, workers: u32, task_base: u32) -> impl ToString {
    let mut dep_graph: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();

    for (dep, target) in input.lines().map(|line| parse_deps(line)) {
        dep_graph
            .entry(target)
            .or_insert(BTreeSet::new())
            .insert(dep);
        dep_graph.entry(dep).or_insert(BTreeSet::new());
    }

    let mut duration: u32 = 0;
    let mut available_workers: u32 = workers;
    let mut execution_timers: BTreeMap<char, u32> = BTreeMap::new();

    while !dep_graph.is_empty() || !execution_timers.is_empty() {
        let new: Vec<char> = dep_graph
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .take(available_workers as usize)
            .map(|(task, _)| task)
            .cloned()
            .collect();
        new.iter().for_each(|&curr| {
            dep_graph.remove(&curr);
            available_workers -= 1;
            execution_timers.insert(curr, (curr as u32) - ASCII_CAP_OFFSET + task_base);
        });

        duration += 1;
        execution_timers.iter_mut().for_each(|(_, timer)| {
            *timer -= 1;
        });
        let old: Vec<char> = execution_timers
            .iter()
            .filter(|(_, &timer)| timer == 0)
            .map(|(task, _)| task)
            .cloned()
            .collect();
        old.iter().for_each(|&curr| {
            dep_graph.iter_mut().for_each(|(_, deps)| {
                deps.remove(&curr);
            });
            available_workers += 1;
            execution_timers.remove(&curr);
        });
    }

    duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process(
                &vec![
                    "Step C must be finished before step A can begin.",
                    "Step C must be finished before step F can begin.",
                    "Step A must be finished before step B can begin.",
                    "Step A must be finished before step D can begin.",
                    "Step B must be finished before step E can begin.",
                    "Step D must be finished before step E can begin.",
                    "Step F must be finished before step E can begin.",
                ]
                .join("\n"),
                2,
                0
            )
            .to_string(),
            "15"
        );
    }
}
