use std::collections::HashMap;

fn main() {
    // Read input
    let contents = include_str!("../input/input.txt");
    // mutable vector to store values
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // Loop through each line and split the values
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }
    // Pass the left and right list to part 1 and 2
    part1(&mut left, &mut right);
    part2(&mut left, &mut right);
    println!("\n    *•.¸♡ 𝒎𝒆𝒓𝒓𝒚 𝒄𝒉𝒓𝒊𝒔𝒕𝒎𝒂𝒔 𝒎𝒆𝒐𝒘 ♡¸.•*🎄");
}

fn part1(left: &mut Vec<i32>, right: &mut Vec<i32>) {
    left.sort();
    right.sort();

    let output = left
        .iter()
        .zip(right.iter()) // zip two lists tgt
        .map(|(x, y)| (x - y).abs()) //apply to each pair
        .sum::<i32>();

    println!("DAY01 Part1: The output is: {}", output);
}

fn part2(left: &mut Vec<i32>, right: &mut Vec<i32>) {
    let mut r_count_map: HashMap<i32, i32> = HashMap::new();
    for num in right.iter() {
        let count = r_count_map.entry(*num).or_insert(0);
        *count += 1;
    }
    
    let mut output = 0;
    for num in left.iter() {
        let count = r_count_map.entry(*num).or_insert(0);  // 0 if no match
        output += num * *count; // num * (*count) 
    }


    println!("DAY01 Part2: The output is: {}", output);
}