fn main() {
    println!("Hello, world!");
    let contents = include_str!("../input/input.txt");
    part1(contents);






}

fn part1(contents: &str) {
    // mul(number, number)


    let mut output = 0;
    // invalid characters that should be ignored, even if they look like part of a mul instruction. 
    //Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

    // For example, consider the following section of corrupted memory:
    
    // xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    // Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
    
    // Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?
    println!("Part 1 result: {}", output);
}