use std::{collections::HashMap, fs, ops::AddAssign};

fn get_elves(file_contents: &String) -> HashMap<i32, i32> {
    let lines = file_contents.split("\n");

    let mut i = 0;
    let mut collector = 0;
    let mut elves: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        match line {
            "" => {
                elves.insert(i, collector);
                collector = 0;
                i.add_assign(1);
            }
            _ => {
                let num: i32 = str::parse(line).unwrap();
                collector.add_assign(num);
            }
        }
    }

    elves
}

fn part_1(elves: &HashMap<i32, i32>) -> i32 {
    let mut totals: Vec<i32> = elves.values().cloned().collect();
    totals.sort_unstable_by(|a, b| b.cmp(a));
    *totals.first().unwrap()
}

fn part_2(elves: &HashMap<i32, i32>) -> i32 {
    let mut totals: Vec<i32> = elves.values().cloned().collect();
    totals.sort_unstable_by(|a, b| b.cmp(a));

    let top_three = totals[..3].to_vec();

    top_three.into_iter().reduce(|a, b| a + b).unwrap()
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/day1.txt").unwrap();
    let elves_map = get_elves(&file_contents);

    let first_answer = part_1(&elves_map);
    let second_answer = part_2(&elves_map);

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);
}
