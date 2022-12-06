use std::{fs, ops::RangeInclusive, str::FromStr};
type IntRange = RangeInclusive<i32>;

fn line_to_pair(line: &str) -> (&str, &str) {
    let (left, right) = line.split_once(",").unwrap();

    (left, right)
}

fn parse_to_range(s: &str) -> IntRange {
    let (left, right) = s.split_once("-").unwrap();

    let start = FromStr::from_str(left).unwrap();
    let end = FromStr::from_str(right).unwrap();

    start..=end
}

fn pair_to_range_pair(pair: (&str, &str)) -> (IntRange, IntRange) {
    (parse_to_range(pair.0), parse_to_range(pair.1))
}

fn contained(range_pair: &(IntRange, IntRange)) -> bool {
    let (first, second) = range_pair.to_owned();

    let in_first = first.contains(second.start()) && first.contains(second.end());
    let in_second = second.contains(first.start()) && second.contains(first.end());

    in_first || in_second
}

fn overlaps(range_pair: &(IntRange, IntRange)) -> bool {
    let (first, second) = range_pair.to_owned();

    let overlaps_first = first.contains(second.start()) || first.contains(second.end());
    let overlaps_second = second.contains(first.start()) || second.contains(first.end());

    overlaps_first || overlaps_second
}

fn part_1(file_str: &str) -> usize {
    file_str
        .split("\n")
        .map(|line| line_to_pair(line))
        .map(|pair| pair_to_range_pair(pair))
        .filter(|range_pair| contained(range_pair))
        .count()
}

fn part_2(file_str: &str) -> usize {
    file_str
        .split("\n")
        .map(|line| line_to_pair(line))
        .map(|pair| pair_to_range_pair(pair))
        .filter(|range_pair| overlaps(range_pair))
        .count()
}

fn main() {
    let file_contents = fs::read_to_string("puzzles/day4.txt").unwrap();
    let file_str = file_contents.trim();

    let first_answer = part_1(file_str);
    println!("First answer: {}", first_answer);

    let second_answer = part_2(file_str);
    println!("Second answer: {}", second_answer);
}
