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
                        if out_of_bound_checking(width, height, next_x, next_y)
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
    let (width, height) = get_input_dimensions(contents);
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    //Check each row
    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] == 'A' {
                // / top right, bottom left
                let (tr_x, tr_y) = (x as isize + 1, y as isize + 1);
                let (bl_x, bl_y) = (x as isize - 1, y as isize - 1);
                // \ top left, bottom right
                let (br_x, br_y) = (x as isize + 1, y as isize - 1);
                let (tl_x, tl_y) = (x as isize - 1, y as isize + 1);
                if check_diagonal(&grid, width, height, tr_x, tr_y, bl_x, bl_y)
                    && check_diagonal(&grid, width, height, br_x, br_y, tl_x, tl_y)
                {
                    count += 1;
                }
            }
        }
    }
    println!("DAY04 Part 2: {}", count);
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

// Check out of bound
fn out_of_bound_checking(width: isize, height: isize, next_x: isize, next_y: isize) -> bool {
    let mut oob = false;
    if next_x < 0  // out of bounds
        || next_x >= width
        || next_y < 0
        || next_y >= height
    {
        oob = true;
    }
    return oob;
}

// Part 2: Check Diagonal
fn check_diagonal(grid: &Vec<Vec<char>>, width: isize, height: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    let mut valid = true;
    // if either of them is oob
    if out_of_bound_checking(width, height, x1, y1) || out_of_bound_checking(width, height, x2, y2)
    {
        valid = false;
    }
    // No M or No S
    else if grid[y1 as usize][x1 as usize] != 'M' && grid[y2 as usize][x2 as usize] != 'M'
        || grid[y1 as usize][x1 as usize] != 'S' && grid[y2 as usize][x2 as usize] != 'S'{
        valid = false;
    }
    // Not SM or Not MS
    else if grid[y1 as usize][x1 as usize] == 'S' && grid[y2 as usize][x2 as usize] != 'M'
        || grid[y1 as usize][x1 as usize] == 'M' && grid[y2 as usize][x2 as usize] != 'S'{
        valid = false;
    }
    return valid;
}
