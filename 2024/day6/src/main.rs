fn main() {
    let contents = include_str!("../input/input.txt");
    println!("\nMaster, input read meow ^. .^\n");
    // let contents = include_str!("../input/ex_input.txt");
    println!("\nMaster, input read meow ^. .^\n");
    part1(contents);
    // part2(contents);
    println!("\n    *•.¸♡ 𝒎𝒆𝒓𝒓𝒚 𝒄𝒉𝒓𝒊𝒔𝒕𝒎𝒂𝒔 𝒎𝒆𝒐𝒘 ♡¸.•*🎄");
}

// part 1
fn part1(contents: &str) {
    /*
        ^ guard start position (facing up)
        # obstacle 
        If there is something directly in front of you, turn right 90 degrees. Otherwise, take a step forward.
     */
    let mut total = 0;
    


    println!("Part 1: {}", total);
}