mod days;

const DAY: i32 = 1;
fn main() {
    if DAY == 1 {
        days::day1::part1().expect("reason");
        days::day1::part2().expect("reason");
    }
}
