use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use csv::Writer;

use solana_price_parser::{parse_blockchain_data, read_lines};

fn main() {
    let output_csv_path = "parsed_output.csv";

    // Create CSV writer
    let mut csv_writer = Writer::from_path(output_csv_path).expect("Unable to create output CSV file");

    if let Ok(lines) = read_lines("data.txt") {
        for line in lines {
            if let Ok(record) = line {
                match parse_blockchain_data(&record) {
                    Ok(entry) => {
                        // Output to CSV file
                        csv_writer.serialize(&entry).expect("Failed to write entry to CSV file");
                        csv_writer.flush().expect("Failed to flush CSV writer");
                    }
                    Err(e) => eprintln!("Failed to parse entry: {}", e),
                }
            }
        }
    }
}


