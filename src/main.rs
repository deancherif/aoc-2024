mod days;

const DAY: i32 = 2;
fn main() {
    if DAY == 1 {
        println!("Part 1: {}", days::day1::part1().expect("reason"));
        println!("Part 2: {}", days::day1::part2().expect("reason"));
    }
    if DAY == 2 {
        println!("Part 1: {}", days::day2::part1().expect("reason"));
        println!("Part 2: {}", days::day2::part2().expect("reason"));
    }
}
