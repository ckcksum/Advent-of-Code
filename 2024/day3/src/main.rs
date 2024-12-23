use regex::Regex;
fn main() {
    let contents = include_str!("../input/input.txt");
    println!("\nMaster, input read meow ^. .^\n");
    part1(contents);
    part2(contents);
    println!("\n    *•.¸♡ 𝒎𝒆𝒓𝒓𝒚 𝒄𝒉𝒓𝒊𝒔𝒕𝒎𝒂𝒔 𝒎𝒆𝒐𝒘 ♡¸.•*🎄");
}

fn part1(contents: &str) {
    // mul(x,y)
    let re = Regex::new(r"mul\(([0-9]{1,3})+,([0-9]{1,3})\)").unwrap();
    println!("DAY03 Part 1: {}", re.captures_iter(contents).map(|cap| {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        x * y
    }).sum::<i32>());
}

fn part2(contents: &str) {
    // mul(x,y)|do()|don't()
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    println!("DAY03 Part 2: {}", re.captures_iter(contents).map(|cap| {
        match &cap[0] {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ if enabled => {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                x * y
            }
            _ => 0,
        }
    }).sum::<i32>());
}