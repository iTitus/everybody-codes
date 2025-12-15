use std::fmt;
use std::str::FromStr;

/// Represents a quest day (1-25 typically)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event {
    Event(u32),
    Story(u32),
}

impl Event {
    pub fn as_u32(&self) -> u32 {
        match self {
            Self::Event(n) | Self::Story(n) => *n,
        }
    }
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("e") {
            Ok(Self::Event(
                s.parse()
                    .map_err(|_| format!("Invalid event number: {s}"))?,
            ))
        } else if let Some(s) = s.strip_prefix("s") {
            Ok(Self::Story(
                s.parse()
                    .map_err(|_| format!("Invalid story number: {s}"))?,
            ))
        } else {
            Err(format!("Invalid event/story {s}"))
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Event(n) => write!(f, "e{n}"),
            Event::Story(n) => write!(f, "s{n}"),
        }
    }
}
