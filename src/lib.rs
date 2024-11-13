use pest::Parser;
use pest_derive::Parser;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct BlockchainParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("File error: {0}")]
    FileError(#[from] io::Error),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub fn parse_entry(input: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut entry_map: HashMap<String, String> = HashMap::new();
    let pairs = BlockchainParser::parse(Rule::entry, input)
        .map_err(|e| ParseError::ParsingError(format!("{:?}", e)))?;

    for pair in pairs {
        if pair.as_rule() == Rule::entry {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::name => {
                        let name = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Name".to_string(), name.to_string());
                    }
                    Rule::date_entry => {
                        let date = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Date".to_string(), date.to_string());
                    }
                    Rule::open => {
                        let open = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Open".to_string(), open.to_string());
                    }
                    Rule::close => {
                        let close = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Close".to_string(), close.to_string());
                    }
                    Rule::high => {
                        let high = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("High".to_string(), high.to_string());
                    }
                    Rule::low => {
                        let low = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Low".to_string(), low.to_string());
                    }
                    Rule::volume => {
                        let volume = inner_pair.into_inner().next().unwrap().as_str();
                        entry_map.insert("Volume".to_string(), volume.to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(entry_map)
}

pub fn parse_file_to_json(input_path: &Path, output_path: &Path) -> Result<(), ParseError> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        match parse_entry(&line) {
            Ok(entry) => entries.push(entry),
            Err(e) => eprintln!("Error parsing line: {}", e),
        }
    }

    let json_data = json!(entries);
    let pretty_json = serde_json::to_string_pretty(&json_data)?;

    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", pretty_json)?;

    Ok(())
}
