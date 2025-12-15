use crate::ec::Event;
use crate::{Client, Quest};
use std::env;
use std::fmt::{Debug, Display};
use std::time::Instant;

pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";
pub const ANSI_GREEN: &str = "\x1b[32m";
pub const ANSI_RED: &str = "\x1b[31m";

pub fn run_part<'a, A: Answer + Sized>(
    func: impl FnOnce(&'a str) -> A + 'a,
    input: &'a str,
    event: &str,
    quest: u8,
    part: u8,
) where
    <A as Answer>::Output: Display,
    <A as Answer>::Error: Debug,
{
    let event: Event = event.parse().expect("invalid event/story");
    let quest: Quest = quest.try_into().expect("invalid quest");

    // Print result inline
    if input.is_empty() {
        println!("Part {}: -", part);
        return;
    }

    let timer = Instant::now();
    let result = func(input);
    let duration = timer.elapsed();

    match result.unpack() {
        Ok(answer) => {
            let answer_str = answer.to_string();
            if answer_str.contains('\n') {
                println!("Part {part}: (multiline) ({duration:?})");
                println!("{}", answer_str);
            } else {
                print!("Part {part}: {ANSI_BOLD}{answer_str}{ANSI_RESET} ({duration:?})",);

                // Check if we should submit and get response inline
                if let Some(submission_info) = check_and_submit(&answer_str, event, quest, part) {
                    print!(" - {submission_info}");
                }

                println!();
            }
        }
        Err(e) => {
            println!("Part {part}: - ({e:?})");
        }
    }
}

pub trait Answer {
    type Output;
    type Error;

    fn unpack(self) -> Result<Self::Output, Self::Error>;
}

impl<T> Answer for Option<T> {
    type Output = T;
    type Error = ();

    fn unpack(self) -> Result<Self::Output, Self::Error> {
        self.ok_or(())
    }
}

impl<T, E> Answer for Result<T, E> {
    type Output = T;
    type Error = E;

    fn unpack(self) -> Result<Self::Output, Self::Error> {
        self
    }
}

fn check_and_submit(result: &str, event: Event, quest: Quest, part: u8) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    // Check if we should submit AND if this is the part to submit
    let should_submit = args
        .iter()
        .position(|x| x == "--submit")
        .and_then(|idx| args.get(idx + 1))
        .and_then(|s| s.parse::<u8>().ok())
        .map(|submit_part| submit_part == part)
        .unwrap_or(false);

    if !should_submit {
        return None;
    }

    match Client::try_new() {
        Ok(client) => match client.submit_answer(event, quest, part, result) {
            Ok(response) => format_submission_response(&response),
            Err(e) => Some(format!(
                "{}✗ Submission failed: {}{}",
                ANSI_RED, e, ANSI_RESET
            )),
        },
        Err(e) => Some(format!("{ANSI_RED}✗ Client error: {e}{ANSI_RESET}")),
    }
}

fn format_submission_response(response: &str) -> Option<String> {
    // Try to parse as JSON
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
        let correct = json
            .get("correct")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if correct {
            let mut parts = vec![format!("{ANSI_GREEN}✓ Correct answer!{ANSI_RESET}")];

            if let Some(global_place) = json.get("globalPlace").and_then(|v| v.as_i64())
                && global_place > 0
            {
                parts.push(format!("Global rank: #{}", global_place));
            }

            Some(parts.join(" - "))
        } else {
            let mut msg = format!("{}✗ Incorrect answer{}", ANSI_RED, ANSI_RESET);

            let length_correct = json
                .get("lengthCorrect")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !length_correct {
                msg.push_str(" (wrong length)");
            }

            Some(msg)
        }
    } else {
        // Fallback to raw response
        Some(response.to_string())
    }
}
