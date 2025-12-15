pub mod ec;

use crate::ec::Event;
use crate::ec::client::ClientError;
pub use ec::{Client, Quest, read_example_file, read_input_file, run_part};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Determines which quest to scaffold based on existing input files
pub fn determine_next_quest(event: Event) -> u8 {
    let inputs_dir = PathBuf::from("inputs/notes");

    for quest in 1..=25 {
        for part in 1..=3 {
            let file_path = inputs_dir.join(format!("{event}-{quest:02}-{part}.txt"));
            if !file_path.exists() {
                return quest;
            }
        }
    }

    // If all exist, default to 1
    1
}

/// Determines which quest to solve based on existing input files
pub fn determine_current_quest(event: Event) -> u8 {
    let inputs_dir = PathBuf::from("inputs/notes");

    let mut last_quest = 1;
    'outer: for quest in 1..=25 {
        for part in 1..=3 {
            let file_path = inputs_dir.join(format!("{event}-{quest:02}-{part}.txt"));
            if !file_path.exists() {
                break 'outer;
            } else {
                last_quest = quest;
            }
        }
    }

    last_quest
}

/// Determines which part to scaffold based on existing input files
pub fn determine_next_part(event: Event, quest: Quest) -> u8 {
    let inputs_dir = PathBuf::from("inputs/notes");

    for part in 1..=3 {
        let file_path = inputs_dir.join(format!("{event}-{quest:02}-{part}.txt"));
        if !file_path.exists() {
            return part;
        }
    }

    // If all exist, default to 1
    1
}

/// Creates the directory structure and files for a quest
pub fn scaffold_quest(
    event: Option<String>,
    quest: Option<u8>,
    part: Option<u8>,
) -> Result<(), Box<dyn Error>> {
    let event: Event = event
        .or_else(|| std::env::var("EC_EVENT").ok())
        .ok_or(ClientError::EventNotConfigured)?
        .parse()?;
    let quest: Quest = quest
        .unwrap_or_else(|| determine_next_quest(event))
        .try_into()?;
    let part = part.unwrap_or_else(|| determine_next_part(event, quest));
    println!("Scaffolding: {event}-{quest:02}-{part}");

    // Try to download input first - if this fails, don't create any files
    let input_content = download_input(event, quest, part)?;

    // Create directories
    fs::create_dir_all("src/bin")?;
    fs::create_dir_all("inputs/notes")?;
    fs::create_dir_all("inputs/examples")?;

    // Create quest file from template if it doesn't exist
    let quest_file = PathBuf::from(format!("src/bin/quest_{event}_{quest:02}.rs"));
    if !quest_file.exists() {
        let template = include_str!("./template.txt");
        let content = template
            .replace("%EVENT%", &format!("\"{event}\""))
            .replace("%QUEST_NUMBER%", &quest.as_u8().to_string());
        fs::write(&quest_file, content)?;
        println!("Created [\x1b[0;32m {} \x1b[0m]", quest_file.display());
    } else {
        println!("Quest file already exists: {}", quest_file.display());
    }

    // Create input file
    let input_file = PathBuf::from(format!("inputs/notes/{event}-{quest:02}-{part}.txt"));
    if !input_file.exists() {
        fs::write(&input_file, input_content)?;
        println!("Created [\x1b[0;32m {} \x1b[0m]", input_file.display());
    } else {
        println!("Input file already exists: {}", input_file.display());
    }

    // Create example file
    let example_file = PathBuf::from(format!("inputs/examples/{event}-{quest:02}-{part}.txt"));
    if !example_file.exists() {
        fs::write(&example_file, "")?;
        println!("Created [\x1b[0;32m {} \x1b[0m]", example_file.display());
    } else {
        println!("Example file already exists: {}", example_file.display());
    }

    println!("\nScaffolding complete for Quest {quest} Part {part}");
    Ok(())
}

/// Downloads and decrypts input for a quest part
fn download_input(event: Event, quest: Quest, part: u8) -> Result<String, Box<dyn Error>> {
    let client = Client::try_new()?;
    let input = client.fetch_and_decrypt_input(event, quest, part)?;
    Ok(input)
}

/// Runs a quest solution
pub fn solve_quest(
    event: Option<String>,
    quest: Option<u8>,
    part: Option<u8>,
    submit: bool,
) -> Result<(), Box<dyn Error>> {
    let event: Event = event
        .or_else(|| std::env::var("EC_EVENT").ok())
        .ok_or(ClientError::EventNotConfigured)?
        .parse()?;
    let quest: Quest = quest
        .unwrap_or_else(|| determine_current_quest(event))
        .try_into()?;
    println!("Solving: {event}-{quest:02}-{part:?}");

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--release")
        .arg("--bin")
        .arg(format!("quest_{event}_{quest:02}"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Add -- separator before custom args
    if submit {
        if let Some(p) = part {
            cmd.arg("--");
            cmd.arg("--submit");
            cmd.arg(p.to_string());
        } else {
            return Err("Must specify a part number to submit".into());
        }
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err("Quest execution failed".into());
    }

    Ok(())
}
