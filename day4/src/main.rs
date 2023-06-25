use std::fs;
use std::io;

#[derive(Debug)]
struct SectionAssignmentPair {
    range1: (usize, usize),
    range2: (usize, usize),
}

fn parse_input(input: &str) -> Vec<SectionAssignmentPair> {
    input
        .lines()
        .map(|line| {
            let ranges: Vec<usize> = line
                .split(',')
                .flat_map(|pair| pair.split('-'))
                .map(|num| num.trim().parse().unwrap())
                .collect();

            SectionAssignmentPair {
                range1: (ranges[0], ranges[1]),
                range2: (ranges[2], ranges[3]),
            }
        })
        .collect()
}

fn is_range_fully_contained(range1: (usize, usize), range2: (usize, usize)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1
}

fn is_range_overlapping(range1: (usize, usize), range2: (usize, usize)) -> bool {
    range1.1 >= range2.0 && range2.1 >= range1.0
}

fn count_fully_contained_pairs(assignment_pairs: &[SectionAssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|&pair| {
            is_range_fully_contained(pair.range1, pair.range2)
                || is_range_fully_contained(pair.range2, pair.range1)
        })
        .count()
}

fn count_overlapping_pairs(assignment_pairs: &[SectionAssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|&pair| {
            is_range_overlapping(pair.range1, pair.range2)
                || is_range_overlapping(pair.range2, pair.range1)
        })
        .count()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let assignments = parse_input(&input);

    let count_fully_contained = count_fully_contained_pairs(&assignments);
    println!(
        "Number of assignment pairs where one range fully contains the other: {}",
        count_fully_contained
    );

    let count_overlapping = count_overlapping_pairs(&assignments);
    println!(
        "Number of assignment pairs where the ranges overlap: {}",
        count_overlapping
    );

    Ok(())
}
