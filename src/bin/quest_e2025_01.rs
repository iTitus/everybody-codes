use std::error::Error;

ec::solution!("e2025", 1);

fn parse(notes: &str) -> Result<(Vec<String>, Vec<isize>), Box<dyn Error>> {
    let (names, instructions) = notes.split_once("\n\n").ok_or("cannot split")?;
    let names: Vec<_> = names.split(",").map(ToString::to_string).collect();
    if names.is_empty() {
        return Err("no names".into());
    }

    Ok((
        names,
        instructions
            .split(",")
            .map(|s| {
                if let Some(n) = s.strip_prefix("L") {
                    Ok(-n.parse::<isize>()?)
                } else if let Some(n) = s.strip_prefix("R") {
                    Ok(n.parse::<isize>()?)
                } else {
                    Err("invalid instruction".into())
                }
            })
            .collect::<Result<Vec<isize>, Box<dyn Error>>>()?,
    ))
}

pub fn part_one(notes: &str) -> Result<String, Box<dyn Error>> {
    let (names, instructions) = parse(notes)?;
    let count = names.len() as isize;

    let mut pos: isize = 0;
    for n in instructions {
        pos = (pos + n).clamp(0, count - 1);
    }

    Ok(names[pos as usize].clone())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Result<String, Box<dyn Error>> {
    let (names, instructions) = parse(notes)?;
    let count = names.len() as isize;

    let mut pos: isize = 0;
    for n in instructions {
        pos = (pos + n).rem_euclid(count);
    }

    Ok(names[pos as usize].clone())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Result<String, Box<dyn Error>> {
    let (mut names, instructions) = parse(notes)?;
    let count = names.len() as isize;

    for n in instructions {
        let target = n.rem_euclid(count);
        names.swap(0, target as usize);
    }

    Ok(names[0].clone())
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
        assert_eq!(result, "Fyrryn");
    }

    #[test]
    fn test_part_two() {
        let notes = read_example_file(EVENT, QUEST, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, "Elarzris");
    }

    #[test]
    fn test_part_three() {
        let notes = read_example_file(EVENT, QUEST, 3);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, "Drakzyph");
    }
}
