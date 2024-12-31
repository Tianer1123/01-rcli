use std::fs;

use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers().unwrap().clone();
    for result in reader.records() {
        let json_value = header.iter().zip(result.unwrap().iter()).collect::<Value>();
        ret.push(json_value);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}
