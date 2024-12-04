use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn daytwo() -> io::Result<()> {
    // Define the file path
    // let file_path = "./testData.txt";
    let file_path = "./smallGood.txt";
    let mut its_good = 0;

    if let Ok(lines) = read_lines(file_path) {
        let line_vec: Vec<String> = lines.filter_map(Result::ok).collect();
        for line in line_vec {
            let current_report: Vec<i32> =
                line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
            println!("Testing {:?}", current_report);
            if check_report_safety_conditions(current_report.clone()) {
                println!("passed{:?}", current_report);

                its_good = its_good + 1;
            };
        }
    }
    println!("Good reports {:?}", its_good);
    Ok(())
}

fn get_lowest_index<T: PartialOrd>(data: &[T]) -> Option<usize> {
    data.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) // Find the minimum element
        .map(|(index, _)| index) // Extract the index of the min element
}

fn check_report_safety_conditions(report_data: Vec<i32>) -> bool {
    if report_data.len() < 2 {
        return false;
    }

    let mut faults = 0;
    let mut first_half = report_data.clone();

    // Vibes based asc detection
    let lowest_index = get_lowest_index(&report_data).unwrap();
    let second_half = first_half.split_off(lowest_index);
    let mut second_half_sorted = second_half.clone();
    let mut second_half_sorted_desc = second_half.clone();
    let mut first_half_desc = first_half.clone();

    second_half_sorted.sort();
    second_half_sorted_desc.sort_by(|a, b| b.cmp(a));

    first_half.sort();
    first_half_desc.sort_by(|a, b| b.cmp(a));
    let fuck_ass_asc = [first_half, second_half_sorted].concat();
    let fuck_ass_desc = [first_half_desc, second_half_sorted_desc].concat();

    // how many places needed to move, we assume the data is 'mostly' sorted and take the lower value of these two
    let direction_test_asc = report_data
        .iter()
        .zip(fuck_ass_asc.iter())
        .filter(|(orig, sorted)| orig != sorted)
        .count();

    let direction_test_desc = report_data
        .iter()
        .zip(fuck_ass_desc.iter())
        .filter(|(orig, sorted)| orig != sorted)
        .count();

    let direction_increasing = direction_test_asc < direction_test_desc;

    let num_pre_one = report_data[0];
    let num_pre_two = report_data[1];
    let mut prev_number = num_pre_one;

    if num_pre_one == num_pre_two {
        faults = faults + 1;
        prev_number = num_pre_two;
    } else if (num_pre_two - num_pre_one).abs() > 3 {
        faults = faults + 1;
        prev_number = num_pre_two;
    }

    let report_clone = report_data;
    report_clone.iter().skip(faults + 1).all(|&row_number| {
        let diff = if direction_increasing {
            println!("increase {}", row_number - prev_number);

            row_number - prev_number
        } else {
            println!("decrease {}", prev_number - row_number);
            println!("el {}", row_number);
            prev_number - row_number
        };
        if diff > 3 || diff < 1 {
            faults = faults + 1;
            println!("dddd {}", prev_number);
            // prev_number = row_number
        } else {
            println!(" {} is now {}", prev_number, row_number);
            prev_number = row_number;
        }
        if faults > 1 {
            return false;
        }

        true
    })
}

// Part one
// fn check_report_safety_conditions(report_data: Vec<i32>) -> bool {
//     if report_data.len() < 2 {
//         return false;
//     }

//     let num_pre_one = report_data[0];
//     let num_pre_two = report_data[1];

//     if num_pre_one == num_pre_two || (num_pre_two - num_pre_one).abs() > 3 {
//         return false;
//     }

//     let direction_increasing = num_pre_one < num_pre_two;

//     let mut prev_number = num_pre_two;

//     report_data.iter().skip(2).all(|&row_number| {
//         let diff = if direction_increasing {
//             row_number - prev_number
//         } else {
//             prev_number - row_number
//         };

//         if !(diff <= 3 && diff >= 1) {
//             return false;
//         }

//         prev_number = row_number;
//         true
//     })
// }

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
