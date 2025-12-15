use std::fmt;
use std::str::FromStr;

/// Represents a quest day (1-25 typically)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quest(u8);

impl Quest {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Quest {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(format!(
                "Quest number must be greater than 0, but was {value}."
            ))
        } else {
            Ok(Self(value))
        }
    }
}

impl FromStr for Quest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s
            .parse::<u8>()
            .map_err(|_| format!("Invalid quest number: {s}"))?;
        day.try_into()
    }
}

impl fmt::Display for Quest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
