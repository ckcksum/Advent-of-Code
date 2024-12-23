fn main() {
    let contents = include_str!("../input/input.txt");
    println!("\nMaster, input read meow ^. .^\n");
    part1(contents);
    part2(contents);
    println!("\n    *•.¸♡ 𝒎𝒆𝒓𝒓𝒚 𝒄𝒉𝒓𝒊𝒔𝒕𝒎𝒂𝒔 𝒎𝒆𝒐𝒘 ♡¸.•*🎄");
}

fn part1(contents: &str) {
    let (width, height) = get_input_dimensions(contents);
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let target = ['X', 'M', 'A', 'S'];
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;

    //check each row
    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] == 'X' {
                // If find X, check all directions from X
                for &(dx, dy) in &directions {
                    let mut xmas_found: bool = true;
                    // Compare with each target char
                    for i in 1..target.len() {
                        // Coordinates of the next char
                        let (next_x, next_y) = (x + dx * i as isize, y + dy * i as isize);
                        if next_x < 0  // out of bounds
                            || next_x >= width
                            || next_y < 0
                            || next_y >= height
                            || grid[next_y as usize][next_x as usize] != target[i]
                            // not == target char
                        {
                            xmas_found = false;
                            break;
                        }
                    }
                    if xmas_found {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("DAY04 Part 1: {}", count);
}

fn part2(contents: &str) {

    // println!("DAY04 Part 2: {}", contents);
    println!("Meow I don't get part 2 :)")
}

fn get_input_dimensions(contents: &str) -> (isize, isize) {
    let width = contents.lines().next().unwrap().len() as isize;
    let height = contents.lines().count() as isize;
    println!(
        "Master, the input is {} wide and {} tall meow!",
        width, height
    );
    (width, height)
}
