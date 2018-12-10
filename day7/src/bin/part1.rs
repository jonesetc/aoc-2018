use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn parse_deps(s: &str) -> (char, char) {
    let chars: Vec<char> = s.chars().collect();
    (chars[5], chars[36])
}

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn process(input: &str) -> impl ToString {
    let mut dep_graph: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();

    for (dep, target) in input.lines().map(|line| parse_deps(line)) {
        dep_graph
            .entry(target)
            .or_insert(BTreeSet::new())
            .insert(dep);
        dep_graph.entry(dep).or_insert(BTreeSet::new());
    }

    let mut execution_order: Vec<char> = Vec::new();
    while !dep_graph.is_empty() {
        let curr = dep_graph
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .min()
            .unwrap()
            .0
            .clone();
        execution_order.push(curr);

        dep_graph.remove(&curr);
        dep_graph.iter_mut().for_each(|(_, deps)| {
            deps.remove(&curr);
        });
    }

    execution_order.iter().collect::<String>()
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
                .join("\n")
            )
            .to_string(),
            "CABDFE"
        );
    }
}
