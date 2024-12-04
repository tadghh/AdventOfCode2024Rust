use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn dayone() -> io::Result<()> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    // Define the file path
    let file_path = "./testData.txt";

    if let Ok(lines) = read_lines(file_path) {
        let line_vec: Vec<String> = lines.filter_map(Result::ok).collect();
        for line in line_vec {
            if let Some((left, right)) = line.split_once("   ") {
                if let (Ok(left_num), Ok(right_num)) =
                    (left.trim().parse::<i32>(), right.trim().parse::<i32>())
                {
                    left_list.push(left_num);
                    right_list.push(right_num);
                }
            }
        }
    }
    let mut totals = Vec::new();
    let mut totals_two = Vec::new();
    left_list.sort();
    right_list.sort();

    left_list.iter().enumerate().for_each(|(i, num)| {
        if let Some(right) = right_list.get(i) {
            totals.push((right - num).abs());
        }

        // part two peepoClap
        totals_two.push(
            right_list
                .iter()
                .filter(|right_num| right_num == &num)
                .count() as i32
                * num,
        );
    });
    println!("Totals: {:?}", totals.iter().sum::<i32>());
    println!("Totals: {:?}", totals_two.iter().sum::<i32>());
    Ok(())
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
