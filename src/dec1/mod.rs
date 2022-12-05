use std::fs;
use std::time::Instant;

fn calculate_largest_x(text: String, x: usize) -> Vec<i64> {
    let mut largest_few = vec![0; x];
    let mut running_total = 0;

    for line in text.lines() {
        let line = line.trim();
        if !line.is_empty() {
            running_total += line.parse::<i64>().unwrap();
            continue;
        }

        if running_total > largest_few[0] {
            largest_few[0] = running_total;
            largest_few.sort();
        }

        running_total = 0;
    }

    largest_few.sort();
    largest_few
}

fn sort_all(text: String) -> Box<Vec<i64>> {
    let mut totals = Vec::new();
    let mut running_total = 0;
    for line in text.lines() {
        let line = line.trim();
        if !line.is_empty() {
            running_total += line.parse::<i64>().unwrap();
            continue;
        }

        totals.push(running_total);
        running_total = 0;
    }

    totals.sort();

    return Box::new(totals);
}

fn part_two() {
    let start = Instant::now();

    let text =
        fs::read_to_string("calories-per-elf.txt".to_string()).expect("failed to read to string");

    // let largest_few = calculate_largest_x(text, 3);
    let all_cals = sort_all(text).to_vec();

    // let sum: i64 = largest_few.iter().sum();
    if all_cals.len() < 3 {
        eprintln!("all_cals is of len {}", all_cals.len());
        return;
    }

    let largest_few: Vec<i64> = all_cals[all_cals.len() - 3..].to_vec();
    let sum: i64 = largest_few.iter().sum();
    println!(
        "The highest 3 calorie-counts elves are holding: {:?}, their sum is {}",
        largest_few, sum
    );

    println!("This took {:?} microseconds", start.elapsed().as_micros());
}
