use crate::Quest;
use crate::ec::Event;
use aes::Aes256;
use aes::cipher::BlockDecryptMut;
use block_padding::Pkcs7;
use cbc::{Decryptor, cipher::KeyIvInit};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

type Aes256CbcDec = Decryptor<Aes256>;

const BASE_URL: &str = "https://everybody.codes";

// Due to CDN issues, it was advised to not use the CDN URL anymore.
// See https://www.reddit.com/r/everybodycodes/comments/1p75qfr/2025_please_update_your_tools/
const CDN_URL: &str = "https://everybody.codes";

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("session not found")]
    SessionNotFound,
    #[error("seed not configured")]
    SeedNotConfigured,
    #[error("event/story not configured")]
    EventNotConfigured,
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Deserialize)]
struct EncryptedInput {
    #[serde(rename = "1")]
    part1_input: Option<String>,
    #[serde(rename = "2")]
    part2_input: Option<String>,
    #[serde(rename = "3")]
    part3_input: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserResponse {
    seed: u32,
}

#[derive(Debug, Deserialize)]
struct QuestResponse {
    #[serde(rename = "key1")]
    part1_key: Option<String>,
    #[serde(rename = "key2")]
    part2_key: Option<String>,
    #[serde(rename = "key3")]
    part3_key: Option<String>,
}

#[derive(Debug, Serialize)]
struct AnswerPayload {
    answer: String,
}

pub struct Client {
    session: String,
    seed: u32,
    http_client: reqwest::blocking::Client,
}

impl Client {
    pub fn try_new() -> Result<Self, ClientError> {
        let session = Self::read_session()?;

        let http_client = reqwest::blocking::Client::builder().build()?;

        let mut client = Client {
            session,
            seed: 0, // Temporary value
            http_client,
        };

        // Check if seed needs to be fetched
        client.seed = match Self::get_seed() {
            Ok(s) => s,
            Err(_) => {
                // Seed not configured or empty, fetch it from API
                let fetched_seed = client.fetch_user_seed()?;
                println!("\n INFO: Fetched your seed from the API: {fetched_seed}.",);
                println!("You can add this to .cargo/config.toml to avoid fetching it each time.");
                println!();
                fetched_seed
            }
        };

        Ok(client)
    }

    fn read_session() -> Result<String, ClientError> {
        for dir in [
            PathBuf::new(),
            std::env::home_dir().ok_or(ClientError::SessionNotFound)?,
        ] {
            let session_path = dir.join(".ec-session");

            if session_path.exists() {
                let session = fs::read_to_string(session_path)?.trim().to_string();
                return Ok(session);
            }
        }

        Err(ClientError::SessionNotFound)
    }

    fn get_seed() -> Result<u32, ClientError> {
        let seed_str = std::env::var("EC_SEED").map_err(|_| ClientError::SeedNotConfigured)?;
        let seed_str = seed_str.trim();

        // Check if it's just whitespace or empty
        if seed_str.is_empty() {
            return Err(ClientError::SeedNotConfigured);
        }

        seed_str.parse().map_err(|_| ClientError::SeedNotConfigured)
    }

    pub fn fetch_user_seed(&self) -> Result<u32, ClientError> {
        let url = format!("{BASE_URL}/api/user/me");
        let response = self
            .http_client
            .get(&url)
            .header("Cookie", format!("everybody-codes={}", self.session))
            .send()?
            .error_for_status()?;

        let user: UserResponse = response
            .json()
            .map_err(|e| ClientError::HttpError(e.to_string()))?;

        Ok(user.seed)
    }

    pub fn fetch_encrypted_input(
        &self,
        event: Event,
        quest: Quest,
        part: u8,
    ) -> Result<String, ClientError> {
        let url = format!(
            "{CDN_URL}/assets/{}/{}/input/{}.json",
            event.as_u32(),
            quest.as_u8(),
            self.seed
        );

        let response = self.http_client.get(&url).send()?.error_for_status()?;
        let inputs: EncryptedInput = response.json()?;

        let encrypted = match part {
            1 => inputs.part1_input,
            2 => inputs.part2_input,
            3 => inputs.part3_input,
            _ => None,
        };
        encrypted
            .ok_or_else(|| ClientError::HttpError(format!("Part {part} not found in response")))
    }

    pub fn fetch_decryption_key(
        &self,
        event: Event,
        quest: Quest,
        part: u8,
    ) -> Result<String, ClientError> {
        let url = format!(
            "{BASE_URL}/api/event/{}/quest/{}",
            event.as_u32(),
            quest.as_u8()
        );

        let response = self
            .http_client
            .get(&url)
            .header("Cookie", format!("everybody-codes={}", self.session))
            .send()?
            .error_for_status()?;

        let quest_data: QuestResponse = response.json()?;

        let key = match part {
            1 => quest_data.part1_key,
            2 => quest_data.part2_key,
            3 => quest_data.part3_key,
            _ => None,
        };

        key.ok_or_else(|| {
            ClientError::HttpError(format!(
                "Key for part {part} not available (possibly not solved yet)"
            ))
        })
    }

    pub fn decrypt_input(&self, encrypted_hex: &str, key: &str) -> Result<String, ClientError> {
        let encrypted_bytes =
            hex::decode(encrypted_hex).map_err(|e| ClientError::DecryptionError(e.to_string()))?;

        let key_bytes = key.as_bytes();
        if key_bytes.len() != 32 {
            return Err(ClientError::DecryptionError(format!(
                "Key must be 32 bytes, got {}",
                key_bytes.len()
            )));
        }

        let iv_bytes = &key_bytes[..16];

        let mut encrypted_clone = encrypted_bytes.clone();
        let decrypted = Aes256CbcDec::new(key_bytes.into(), iv_bytes.into())
            .decrypt_padded_mut::<Pkcs7>(&mut encrypted_clone)
            .map_err(|e| ClientError::DecryptionError(format!("Decryption failed: {e:?}")))?;

        String::from_utf8(decrypted.to_vec())
            .map_err(|e| ClientError::DecryptionError(e.to_string()))
    }

    pub fn fetch_and_decrypt_input(
        &self,
        event: Event,
        quest: Quest,
        part: u8,
    ) -> Result<String, ClientError> {
        let encrypted = self.fetch_encrypted_input(event, quest, part)?;
        let key = self.fetch_decryption_key(event, quest, part)?;
        self.decrypt_input(&encrypted, &key)
    }

    pub fn submit_answer(
        &self,
        event: Event,
        quest: Quest,
        part: u8,
        answer: impl Into<String>,
    ) -> Result<String, ClientError> {
        let url = format!(
            "{BASE_URL}/api/event/{}/quest/{}/part/{part}/answer",
            event.as_u32(),
            quest.as_u8()
        );

        let payload = AnswerPayload {
            answer: answer.into(),
        };

        let response = self
            .http_client
            .post(&url)
            .header("Cookie", format!("everybody-codes={}", self.session))
            .json(&payload)
            .send()?
            .error_for_status()?;

        Ok(response.text()?)
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}
