use itertools::Itertools;
use std::cmp::Ordering;
use std::error::Error;

ec::solution!("e2025", 5);

#[derive(Default)]
struct FishboneSegment {
    spine: u64,
    left: Option<u64>,
    right: Option<u64>,
}

impl FishboneSegment {
    fn new(spine: u64) -> Self {
        Self {
            spine,
            left: None,
            right: None,
        }
    }

    fn score(&self) -> u64 {
        let s = self.left.map(|n| n.to_string()).unwrap_or_default()
            + &self.spine.to_string()
            + &self.right.map(|n| n.to_string()).unwrap_or_default();
        s.parse().unwrap()
    }
}

#[derive(Default)]
struct Fishbone {
    segments: Vec<FishboneSegment>,
}

impl Fishbone {
    fn add(&mut self, n: u64) {
        for segment in &mut self.segments {
            if n < segment.spine && segment.left.is_none() {
                segment.left = Some(n);
                return;
            } else if n > segment.spine && segment.right.is_none() {
                segment.right = Some(n);
                return;
            }
        }

        self.segments.push(FishboneSegment::new(n));
    }

    fn quality(&self) -> u64 {
        self.segments
            .iter()
            .map(|s| s.spine)
            .join("")
            .parse()
            .unwrap()
    }
}

fn parse(notes: &str) -> Result<(u64, Vec<u64>), Box<dyn Error>> {
    let (identifier, list) = notes.split_once(':').ok_or("No identifier")?;
    Ok((
        identifier.parse()?,
        list.split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn parse_multi(notes: &str) -> Result<Vec<(u64, Vec<u64>)>, Box<dyn Error>> {
    notes
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
}

pub fn part_one(notes: &str) -> Result<u64, Box<dyn Error>> {
    let (_, list) = parse(notes)?;

    let mut fb = Fishbone::default();
    for n in list {
        fb.add(n);
    }

    Ok(fb.quality())
}

pub fn part_two(notes: &str) -> Result<u64, Box<dyn Error>> {
    let swords = parse_multi(notes)?;

    let (min, max) = swords
        .into_iter()
        .map(|(_, list)| {
            let mut fb = Fishbone::default();
            for n in list {
                fb.add(n);
            }
            fb.quality()
        })
        .minmax()
        .into_option()
        .ok_or("no elements")?;

    Ok(max - min)
}

pub fn part_three(notes: &str) -> Result<u64, Box<dyn Error>> {
    let mut swords = parse_multi(notes)?
        .into_iter()
        .map(|(identifier, list)| {
            let mut fb = Fishbone::default();
            for n in list {
                fb.add(n);
            }
            let q = fb.quality();
            (identifier, fb, q)
        })
        .collect::<Vec<_>>();
    swords.sort_unstable_by(|(identifier_1, fb_1, q_1), (identifier_2, fb_2, q_2)| {
        q_1.cmp(q_2)
            .then_with(|| {
                for (s_1, s_2) in fb_1.segments.iter().zip(fb_2.segments.iter()) {
                    let ord = s_1.score().cmp(&s_2.score());
                    if !ord.is_eq() {
                        return ord;
                    }
                }
                Ordering::Equal
            })
            .then_with(|| identifier_1.cmp(identifier_2))
            .reverse()
    });

    let checksum = swords
        .into_iter()
        .enumerate()
        .map(|(i, (identifier, _, _))| ((i as u64) + 1) * identifier)
        .sum();
    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::ec::runner::Answer;
    use ec::{read_example_file, read_numbered_example_file};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let notes = read_example_file(EVENT, QUEST, 1);
        let result = part_one(&notes).unpack().unwrap();
        assert_eq!(result, 581078);
    }

    #[test]
    fn test_part_two() {
        let notes = read_example_file(EVENT, QUEST, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, 77053);
    }

    #[test]
    fn test_part_three() {
        let notes = read_example_file(EVENT, QUEST, 3);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 260);
    }

    #[test]
    fn test_part_three_2() {
        let notes = read_numbered_example_file(EVENT, QUEST, 3, 2);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 4);
    }
}
