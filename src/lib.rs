use pest::Parser;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};
use std::io::BufRead;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct BlockchainParser;

#[derive(Debug, serde::Serialize)]
pub struct BlockchainEntry {
    name: String,
    date: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse entry: {0}")]
    ParseError(String),

    #[error("Missing expected data in parsed structure")]
    MissingData,

    #[error("Failed to parse float value: {0}")]
    FloatParseError(#[from] std::num::ParseFloatError),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Pest parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

impl BlockchainEntry {
    fn from_parsed_data(parsed: pest::iterators::Pair<Rule>) -> Result<Self, ParseError> {
        let mut name = String::new();
        let mut date = String::new();
        let mut open = 0.0;
        let mut close = 0.0;
        let mut high = 0.0;
        let mut low = 0.0;
        let mut volume = 0.0;

        for pair in parsed.into_inner() {
            match pair.as_rule() {
                Rule::blockchain_name => name = pair.as_str().to_string(),
                Rule::date => date = pair.as_str().to_string(),
                Rule::number => {
                    if let Some(label) = pair.clone().into_inner().next() {
                        match label.as_str() {
                            "Open" => open = pair.as_str().parse().map_err(ParseError::FloatParseError)?,
                            "Close" => close = pair.as_str().parse().map_err(ParseError::FloatParseError)?,
                            "High" => high = pair.as_str().parse().map_err(ParseError::FloatParseError)?,
                            "Low" => low = pair.as_str().parse().map_err(ParseError::FloatParseError)?,
                            "Volume" => volume = pair.as_str().parse().map_err(ParseError::FloatParseError)?,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(BlockchainEntry { name, date, open, close, high, low, volume })
    }
}

pub fn parse_blockchain_data(input: &str) -> Result<BlockchainEntry, ParseError> {
    let parsed = BlockchainParser::parse(Rule::entry, input)?.next().ok_or(ParseError::ParseError("Parsing failed".into()))?;
    BlockchainEntry::from_parsed_data(parsed)
}

pub fn write_to_file(entry: &BlockchainEntry, output_path: &str) -> Result<(), ParseError> {
    let mut file = File::create(output_path)?;
    writeln!(file, "Parsed entry: {:?}", entry).map_err(ParseError::IoError)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
