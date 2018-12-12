use std::cell::RefCell;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn sum_metadata(tree: &RefCell<VecDeque<u32>>) -> u32 {
    let children_count = tree.borrow_mut().pop_front().unwrap();
    let metadata_count = tree.borrow_mut().pop_front().unwrap();

    let mut total: u32 = 0;

    if children_count == 0 {
        for _ in 0..metadata_count {
            total += tree.borrow_mut().pop_front().unwrap();
        }
    } else {
        let mut children: Vec<u32> = Vec::new();

        for _ in 0..children_count {
            children.push(sum_metadata(tree));
        }

        for _ in 0..metadata_count {
            let index = tree.borrow_mut().pop_front().unwrap();
            total += children
                .get(index as usize - 1)
                .cloned()
                .unwrap_or_default();
        }
    }

    total
}

fn process(input: &str) -> impl ToString {
    sum_metadata(&RefCell::new(
        input
            .split_whitespace()
            .map(|line| line.parse().unwrap())
            .collect::<VecDeque<u32>>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2").to_string(),
            "66"
        );
    }
}
