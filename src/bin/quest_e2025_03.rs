use itertools::Itertools;
use std::cmp::Reverse;
use std::error::Error;

ec::solution!("e2025", 3);

pub fn part_one(notes: &str) -> Result<u32, Box<dyn Error>> {
    let nums = notes
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(nums.into_iter().unique().sum::<u32>())
}

pub fn part_two(notes: &str) -> Result<u32, Box<dyn Error>> {
    let nums = notes
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    let sorted_unique_nums = nums
        .into_iter()
        .sorted_unstable_by_key(|n| Reverse(*n))
        .dedup()
        .collect::<Vec<_>>();
    let n = sorted_unique_nums.len();
    assert!(n >= 20);
    Ok(sorted_unique_nums[n - 20..].iter().sum::<u32>())
}

pub fn part_three(notes: &str) -> Result<usize, Box<dyn Error>> {
    let nums = notes
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(*nums.iter().counts().values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::ec::runner::Answer;
    use ec::read_example_file;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let notes = read_example_file(EVENT, QUEST, 1);
        let result = part_one(&notes).unpack().unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    fn test_part_two() {
        let notes = read_example_file(EVENT, QUEST, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, 781);
    }

    #[test]
    fn test_part_three() {
        let notes = read_example_file(EVENT, QUEST, 3);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 3);
    }
}
