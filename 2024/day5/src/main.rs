fn main() {
    let contents = include_str!("../input/input.txt");
    println!("\nMaster, input read meow ^. .^\n");
    let (rules, updates) = parse_input(contents); // (rules, updates)
    let (correct_updates, incorrect_updates) = check_updates(&rules, &updates);
    part1(&correct_updates);
    part2(&rules, &incorrect_updates);
    println!("\n    *•.¸♡ 𝒎𝒆𝒓𝒓𝒚 𝒄𝒉𝒓𝒊𝒔𝒕𝒎𝒂𝒔 𝒎𝒆𝒐𝒘 ♡¸.•*🎄");
}

fn part1(correct_updates: &Vec<Vec<i32>>) {
    let mut middle_sum = 0;
    for update in correct_updates {
        middle_sum += update[update.len() / 2];
    }
    println!("DAY05 Part 1: {}", middle_sum);
}

fn part2(rules: &Vec<(i32, i32)>, incorrect_updates: &Vec<Vec<i32>>) {
    let mut reordered_updates = incorrect_updates.clone(); //& is not mutable
    for update in reordered_updates.iter_mut() {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..update.len() - 1 {
                let current = update[i];
                let next = update[i + 1];
                for &(x, y) in rules {
                    if current == y && next == x {
                        update.swap(i, i + 1);
                        changed = true;
                    }
                }
            }
        }
    }
    let mut middle_sum = 0;
    for update in reordered_updates {
        middle_sum += update[update.len() / 2];
    }
    println!("DAY05 Part 2: {}", middle_sum);
}

fn parse_input(contents: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_str, updates_str) = contents.split_once("\n\n").unwrap();
    // Rules: (x,y)
    let rules: Vec<(i32, i32)> = rules_str
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    // Updates: page update rows
    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn check_updates(
    rules: &Vec<(i32, i32)>,
    updates: &Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut correct_updates = Vec::new();
    let mut incorrect_updates = Vec::new();

    for update in updates {
        let mut correct_order = true;
        for i in 0..update.len() - 1 {
            let current = update[i];
            let next = update[i + 1];
            for (x, y) in rules {
                if current == *y && next == *x {
                    correct_order = false;
                    break;
                }
            }
        }
        if correct_order {
            correct_updates.push(update.clone());
        } else {
            incorrect_updates.push(update.clone());
        }
    }
    (correct_updates, incorrect_updates)
}
