fn main() {
    println!("Hello, world!");
    // Read input
    let contents = include_str!("../input/input.txt");
    part1(contents);
    part2(contents);
}

// Each row in input data: 7 6 4 2 1 for example

fn part1(contents: &str) {
    // 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    // 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    // 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    // 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
    let mut safe_counts = 0;
    for row in contents.lines() {
        // split the row into a vector of integers
        let row_vec: Vec<i32> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut safe = true;
        // Check row is increasing, decreasing or same
        if row_vec[0] < row_vec[1] {
            // increasing: curr>prev; curr_prev_diff > 0 && curr_prev_diff <= 3
            for i in 1..row_vec.len() {
                let curr_prev_diff = row_vec[i] - row_vec[i - 1];
                if !(curr_prev_diff > 0 && curr_prev_diff <= 3) {
                    safe = false;
                    break;
                }
            }
        } else if row_vec[0] > row_vec[1] {
            // decreasing: prev>curr; curr_prev_diff < 0 && curr_prev_diff >= -3
            for i in 1..row_vec.len() {
                let curr_prev_diff = row_vec[i] - row_vec[i - 1];
                if !(curr_prev_diff < 0 && curr_prev_diff >= -3) {
                    safe = false;
                    break;
                }
            }
        } else {
            if row_vec[0] == row_vec[2] {
                safe = false;
            }
        }

        if safe {
            safe_counts += 1;
        }
    }
    println!("DAY02 Part1: The output is: {}", safe_counts);
}

fn part2(contents: &str) {
    // 7 6 4 2 1: Safe without removing any level.
    // 1 2 7 8 9: Unsafe regardless of which level is removed.
    // 9 7 6 2 1: Unsafe regardless of which level is removed.
    // 1 3 2 4 5: Safe by removing the second level, 3.
    // 8 6 4 4 1: Safe by removing the third level, 4.
    // 1 3 6 7 9: Safe without removing any level.
    let mut safe_counts = 0;
    for row in contents.lines() {
        // split the row into a vector of integers
        let row_vec: Vec<i32> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut safe = true;
        // Check row is increasing, decreasing or same
        if row_vec[0] < row_vec[1] {
            // increasing: curr>prev; curr_prev_diff > 0 && curr_prev_diff <= 3
            check_increasing_row(row_vec, &mut safe);
        } else if row_vec[0] > row_vec[1] {
            // decreasing: prev>curr; curr_prev_diff < 0 && curr_prev_diff >= -3
            check_decreasing_row(row_vec, &mut safe);
        } else {
            if row_vec[0] == row_vec[2] {
                safe = false;
            } else if row_vec[0] > row_vec[2]{
                check_decreasing_row(row_vec, &mut safe);
            }
            else {
                check_increasing_row(row_vec, &mut safe);
            }
        }

        if safe {
            safe_counts += 1;
        }
    }
    println!("DAY02 Part2: The output is: {}", safe_counts);
}

fn check_increasing_row(row_vec: Vec<i32>, safe: &mut bool) {
    for i in 1..row_vec.len() {
        let curr_prev_diff = row_vec[i] - row_vec[i - 1];
        if !(curr_prev_diff > 0 && curr_prev_diff <= 3) {
            // next > prev
            if i < row_vec.len() - 1 { // if not the last element in row
                let curr_prev_diff = row_vec[i + 1] - row_vec[i - 1];
                if !(curr_prev_diff > 0 && curr_prev_diff <= 3) {
                    *safe = false;
                    break;
                }
            }

        }
    }
}

fn check_decreasing_row(row_vec: Vec<i32>, safe: &mut bool) {
    for i in 1..row_vec.len() {
        let curr_prev_diff = row_vec[i] - row_vec[i - 1];
        if !(curr_prev_diff < 0 && curr_prev_diff >= -3) {
            // next < prev
            if i < row_vec.len() - 1 { // if not the last element in row
                let curr_prev_diff = row_vec[i + 1] - row_vec[i - 1];
                if !(curr_prev_diff < 0 && curr_prev_diff >= -3) {
                    *safe = false;
                    break;
                }
            }
        }
    }
}

