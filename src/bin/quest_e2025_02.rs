use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, RangeInclusive};
use std::str::FromStr;

ec::solution!("e2025", 2);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Complex(i64, i64);

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl FromStr for Complex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('[').ok_or(())?;
        let (r, i) = s.split_once(',').ok_or(())?;
        Ok(Self(
            r.trim().parse().map_err(|_| ())?,
            i.strip_suffix(']')
                .ok_or(())?
                .trim()
                .parse()
                .map_err(|_| ())?,
        ))
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        // [X1,Y1] + [X2,Y2] = [X1 + X2, Y1 + Y2]
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        // [X1,Y1] * [X2,Y2] = [X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2]
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        // [X1,Y1] / [X2,Y2] = [X1 / X2, Y1 / Y2]
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

pub fn part_one(notes: &str) -> Option<Complex> {
    let (_, a) = notes.trim().split_once('=')?;
    let a: Complex = a.parse().ok()?;

    const DIVISOR: Complex = Complex(10, 10);

    let mut r = Complex(0, 0);
    for _ in 0..3 {
        r = (r * r) / DIVISOR + a;
    }

    Some(r)
}

fn check_grid(a: Complex, grid_size: i64) -> usize {
    let b = a + Complex(1000, 1000);

    let step_x = (b.0 - a.0) / (grid_size - 1);
    let step_y = (b.1 - a.1) / (grid_size - 1);

    fn should_engrave(p: Complex) -> bool {
        const DIVISOR: Complex = Complex(100000, 100000);
        const RANGE: RangeInclusive<i64> = -1000000..=1000000;

        let mut r = Complex(0, 0);
        for _ in 0..100 {
            r = (r * r) / DIVISOR + p;

            if !RANGE.contains(&r.0) || !RANGE.contains(&r.1) {
                return false;
            }
        }

        true
    }

    let mut count = 0;
    let mut p = a;
    for _ in 0..grid_size {
        for _ in 0..grid_size {
            if should_engrave(p) {
                count += 1;
            }

            p.0 += step_x;
        }

        p.0 = a.0;
        p.1 += step_y;
    }

    count
}

pub fn part_two(notes: &str) -> Option<usize> {
    let (_, a) = notes.trim().split_once('=')?;
    let a: Complex = a.parse().ok()?;
    Some(check_grid(a, 101))
}

pub fn part_three(notes: &str) -> Option<usize> {
    let (_, a) = notes.trim().split_once('=')?;
    let a: Complex = a.parse().ok()?;
    Some(check_grid(a, 1001))
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
        assert_eq!(result, "[357,862]".parse::<Complex>().unwrap());
    }

    #[test]
    fn test_part_two() {
        let notes = read_example_file(EVENT, QUEST, 2);
        let result = part_two(&notes).unpack().unwrap();
        assert_eq!(result, 4076);
    }

    #[test]
    fn test_part_three() {
        let notes = read_example_file(EVENT, QUEST, 3);
        let result = part_three(&notes).unpack().unwrap();
        assert_eq!(result, 406954);
    }
}
