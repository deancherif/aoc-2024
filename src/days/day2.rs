use std::fs;
use std::io::Result;

pub fn part1() -> Result<i32> {
    let filename = "src/input/day2.txt";
    let contents = fs::read_to_string(filename)?;

    let mut reports: Vec<Vec<u8>> = Vec::new();
    for levels in contents.split("\n") {
        let mut current_level = Vec::new();
        for level in levels.split(" ") {
            let l = level.parse::<u8>();
            if let Ok(parsed) = l {
                current_level.push(parsed)
            };
        }

        reports.push(current_level)
    }

    let mut safe_count = 0;

    for levels in reports.iter() {
        if levels.is_empty() {
            continue;
        }
        let mut safe = true;
        let direction = if levels[0] as i16 - levels[1] as i16 >= 0 {
            -1
        } else {
            1
        };
        println!("{:?} ({})", levels, direction);
        for (j, level) in levels.iter().enumerate() {
            if j > 0
                && !(((direction == 1 && levels[j - 1] < *level)
                    || (direction == -1 && levels[j - 1] > *level))
                    && (levels[j - 1] as i16 - *level as i16).abs() <= 3)
            {
                println!(
                    "-- ({},{}) => ({} || {}) && {}",
                    levels[j - 1],
                    *level,
                    (direction == 1 && levels[j - 1] < *level),
                    (direction == -1 && levels[j - 1] > *level),
                    (levels[j - 1] as i16 - *level as i16).abs() <= 3
                );
                safe = false;
            }
        }

        println!("{}", safe);

        if safe {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

pub fn part2() -> Result<i32> {
    //let filename = "src/input/day2.txt";
    //let contents = fs::read_to_string(filename)?;
    Ok(0)
}
