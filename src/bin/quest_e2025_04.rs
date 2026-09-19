use num::One;
use num::rational::Ratio;
use std::error::Error;
use std::num::ParseIntError;

ec::solution!("e2025", 4);

fn parse_and_get_ratio(notes: &str) -> Result<Ratio<u64>, Box<dyn Error>> {
    let gears = notes
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let result = if let Some((a, b)) = s.split_once('|') {
                let a: u64 = a.parse()?;
                let b: u64 = b.parse()?;
                (a, b)
            } else {
                let n: u64 = s.parse()?;
                (n, n)
            };
            Ok::<_, ParseIntError>(result)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut ratio = Ratio::<u64>::one();
    for &[(_, a), (b, _)] in gears.array_windows() {
        ratio *= Ratio::new(a, b);
    }

    Ok(ratio)
}

pub fn part_one(notes: &str) -> Result<u64, Box<dyn Error>> {
    let ratio = parse_and_get_ratio(notes)?;
    let result = ratio * 2025;
    Ok(result.to_integer())
}

pub fn part_two(notes: &str) -> Result<u64, Box<dyn Error>> {
    let ratio = parse_and_get_ratio(notes)?;
    let result = ratio.recip() * 10000000000000;
    Ok(result.ceil().to_integer())
}

pub fn part_three(notes: &str) -> Result<u64, Box<dyn Error>> {
    let ratio = parse_and_get_ratio(notes)?;
    let result = ratio * 100;
    Ok(result.to_integer())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::ec::runner::Answer;
    use ec::{read_example_file, read_numbered_example_file};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one_1() {
        let notes = read_example_file(EVENT, QUEST, 1);
        let result = part_one(&notes).unpack().unwrap();
        assert_eq!(result, 32400);
    }

    #[test]
    fn test_part_one_2() {
        let notes = read_numbered_example_file(EVENT, QUEST, 1, 2);
        let result = part_one(&notes).unpack().unwrap();
        assert_eq!(result, 15888);
    }

    #[test]
    fn test_part_two() {
        let notes = read_example_file(EVENT, QUEST, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, 625000000000);
    }

    #[test]
    fn test_part_two_2() {
        let notes = read_numbered_example_file(EVENT, QUEST, 2, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, 1274509803922);
    }

    #[test]
    fn test_part_three() {
        let notes = read_example_file(EVENT, QUEST, 3);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 400);
    }

    #[test]
    fn test_part_three_2() {
        let notes = read_numbered_example_file(EVENT, QUEST, 3, 2);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 6818);
    }
}
