use std::{
    collections::HashMap,
    fs,
    ops::{Add, AddAssign, RangeInclusive},
};

fn get_letter_map(range: RangeInclusive<char>, offset: i32) -> HashMap<char, i32> {
    let letter_tuples: Vec<(char, i32)> = range
        .enumerate()
        .map(|(i, letter)| (letter, i as i32 + offset))
        .collect();

    letter_tuples
        .iter()
        .map(|(c, i)| (c.to_owned(), i.to_owned()))
        .collect()
}

fn split_compartments(rucksack: &str) -> (&str, &str) {
    let size = rucksack.len() / 2;

    let (left, right) = rucksack.split_at(size);

    (left, right)
}

fn find_duplicate((left, right): (&str, &str)) -> char {
    let mut count_map: HashMap<char, (i32, i32)> = HashMap::new();

    left.chars().for_each(|c| {
        count_map
            .entry(c)
            .and_modify(|e| e.0.add_assign(1))
            .or_insert((1, 0));
    });

    right.chars().for_each(|c| {
        count_map
            .entry(c)
            .and_modify(|e| e.1.add_assign(1))
            .or_insert((0, 1));
    });

    count_map
        .iter()
        .find(|e| {
            let (left, right) = e.1;

            left.to_owned() >= 1 && right.to_owned() >= 1
        })
        .unwrap()
        .0
        .to_owned()
}

fn find_duplicate_of_three((first, second, third): (&str, &str, &str)) -> char {
    let mut count_map: HashMap<char, (i32, i32, i32)> = HashMap::new();

    first.chars().for_each(|c| {
        count_map
            .entry(c)
            .and_modify(|e| e.0.add_assign(1))
            .or_insert((1, 0, 0));
    });

    second.chars().for_each(|c| {
        count_map
            .entry(c)
            .and_modify(|e| e.1.add_assign(1))
            .or_insert((0, 1, 0));
    });

    third.chars().for_each(|c| {
        count_map
            .entry(c)
            .and_modify(|e| e.2.add_assign(1))
            .or_insert((0, 0, 1));
    });

    count_map
        .iter()
        .find(|e| {
            let (first, second, third) = e.1;

            first.to_owned() >= 1 && second.to_owned() >= 1 && third.to_owned() >= 1
        })
        .unwrap()
        .0
        .to_owned()
}

fn char_to_priority(c: &char) -> i32 {
    let lowercase = get_letter_map('a'..='z', 1);
    let uppercase = get_letter_map('A'..='Z', 27);

    let maybe_lowercase = lowercase.get(c);

    match maybe_lowercase {
        Some(priority) => *priority,
        _ => *uppercase.get(c).unwrap(),
    }
}

fn part_1(file_str: &str) -> i32 {
    file_str
        .split("\n")
        .map(|rucksack| split_compartments(rucksack))
        .map(|compartments| find_duplicate(compartments))
        .map(|c| char_to_priority(&c))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part_2(file_str: &str) -> i32 {
    let lines: Vec<&str> = file_str.split("\n").collect();

    lines
        .chunks(3)
        .map(|slice| (slice[0], slice[1], slice[2]))
        .map(|chunk| find_duplicate_of_three(chunk))
        .map(|c| char_to_priority(&c))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn main() {
    let file_contents = fs::read_to_string("puzzles/day3.txt").unwrap();
    let file_str = file_contents.trim();

    let first_answer = part_1(file_str);
    println!("First answer: {}", first_answer);

    let second_answer = part_2(file_str);
    println!("Second answer: {}", second_answer);
}
