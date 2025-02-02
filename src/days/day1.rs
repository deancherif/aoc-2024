use std::fs;
use std::io::Result;

pub fn part1() -> Result<()> {
    let filename = "src/input/day1.txt";
    let contents = fs::read_to_string(filename)?;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split("   ");
        v1.push(parts.next().unwrap().parse::<i32>().unwrap());
        v2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }
    v1.sort();
    v2.sort();

    let mut acc = 0;
    for i in 0..v2.len() {
        acc += (v1[i] - v2[i]).abs();
    }
    println!("{:?}", acc);
    Ok(())
}

pub fn part2() -> Result<()> {
    let filename = "src/input/day1.txt";
    let contents = fs::read_to_string(filename)?;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split("   ");
        v1.push(parts.next().unwrap().parse::<i32>().unwrap());
        v2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }
    v1.sort();
    v2.sort();

    let mut acc = 0;
    (0..v1.len()).for_each(|i| {
        let mut similarity = 0;
        (0..v2.len()).for_each(|j| {
            if v2[j] == v1[i] {
                similarity += 1;
            }
        });
        acc += v1[i] * similarity;
    });
    println!("{:?}", acc);
    Ok(())
}
