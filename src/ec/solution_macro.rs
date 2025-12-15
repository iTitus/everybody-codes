use std::{env, fs};

/// Helper function that reads an input file to a string.
/// Returns an empty string if the file doesn't exist.
#[must_use]
pub fn read_input_file(event: impl AsRef<str>, quest: u8, part: u8) -> String {
    let event = event.as_ref();
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("inputs")
        .join("notes")
        .join(format!("{event}-{quest:02}-{part}.txt"));

    fs::read_to_string(&filepath).unwrap_or_default()
}

/// Helper function that reads an example file to a string.
#[must_use]
pub fn read_example_file(event: impl AsRef<str>, quest: u8, part: u8) -> String {
    let event = event.as_ref();
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("inputs")
        .join("examples")
        .join(format!("{event}-{quest:02}-{part}.txt"));
    fs::read_to_string(filepath).expect("could not open example file")
}

/// Creates the solution macro for quest binaries
#[macro_export]
macro_rules! solution {
    ($event:expr, $quest:expr) => {
        $crate::solution!(@impl $event, $quest, [part_one, 1] [part_two, 2] [part_three, 3]);
    };
    ($event:expr, $quest:expr, 1) => {
        $crate::solution!(@impl $event, $quest, [part_one, 1]);
    };
    ($event:expr, $quest:expr, 2) => {
        $crate::solution!(@impl $event, $quest, [part_two, 2]);
    };
    ($event:expr, $quest:expr, 3) => {
        $crate::solution!(@impl $event, $quest, [part_three, 3]);
    };

    (@impl $event:expr, $quest:expr, $( [$func:expr, $part:expr] )*) => {
        pub const EVENT: &str = $event;
        pub const QUEST: u8 = $quest;

        fn main() {
            use ec::{run_part, read_input_file};
            $(
                let input = read_input_file(EVENT, QUEST, $part);
                run_part($func, &input, EVENT, QUEST, $part);
            )*

            println!();
        }
    };
}
