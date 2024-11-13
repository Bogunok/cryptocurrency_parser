#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use cryptocurrency_parser::BlockchainParser;
    use cryptocurrency_parser::Rule;
    use pest::Parser;

    #[test]
    fn test_digit() -> Result<()> {
        let valid_input = "5";
        match BlockchainParser::parse(Rule::digit, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed digit: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the digit rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "a";
        match BlockchainParser::parse(Rule::digit, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the digit rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_year() -> Result<()> {
        let valid_input = "2023";
        match BlockchainParser::parse(Rule::year, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed year: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the year rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "20a3";
        match BlockchainParser::parse(Rule::year, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the year rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_month() -> Result<()> {
        let valid_input = "09";
        match BlockchainParser::parse(Rule::month, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed month: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the month rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "december";
        match BlockchainParser::parse(Rule::month, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the month rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_day() -> Result<()> {
        let valid_input = "15";
        match BlockchainParser::parse(Rule::day, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed day: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the day rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "fifteen";
        match BlockchainParser::parse(Rule::day, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the day rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_date() -> Result<()> {
        let valid_input_dot_format = "31.12.2023";
        let valid_input_slash_format = "12/31/2023";

        match BlockchainParser::parse(Rule::date, valid_input_dot_format) {
            Ok(_) => {
                println!("Parsing succeeded for dot-separated date format");
                assert!(
                    true,
                    "Expected valid input to match the dot-separated date format"
                );
            }
            Err(e) => {
                return Err(anyhow!(
                    "Parsing failed for dot-separated date format: {:?}",
                    e
                ));
            }
        }

        match BlockchainParser::parse(Rule::date, valid_input_slash_format) {
            Ok(_) => {
                println!("Parsing succeeded for slash-separated date format");
                assert!(
                    true,
                    "Expected valid input to match the slash-separated date format"
                );
            }
            Err(e) => {
                return Err(anyhow!(
                    "Parsing failed for slash-separated date format: {:?}",
                    e
                ));
            }
        }

        let invalid_input = "31-12-2023";
        match BlockchainParser::parse(Rule::date, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to not match the date rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_blockchain_name() -> Result<()> {
        let valid_input = "BTC";
        match BlockchainParser::parse(Rule::blockchain_name, valid_input) {
            Ok(_) => {
                println!("Parsing succeeded for valid input");
                assert!(
                    true,
                    "Expected valid input to match the blockchain_name rule"
                );
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "XRP";
        match BlockchainParser::parse(Rule::blockchain_name, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(
                    true,
                    "Expected invalid input to not match the blockchain_name rule"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_currency() -> Result<()> {
        let valid_input = "UAH";
        match BlockchainParser::parse(Rule::currency, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed currency: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the currency rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "GBP";
        match BlockchainParser::parse(Rule::currency, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the currency rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let valid_input = "1,234,567,899";
        match BlockchainParser::parse(Rule::number, valid_input) {
            Ok(_) => {
                println!("Parsing succeeded for valid input");
                assert!(true, "Expected valid input to match the number rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = ".67";
        match BlockchainParser::parse(Rule::number, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to not match the number rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_number_and_currency() -> Result<()> {
        let valid_input = "100 USD";
        match BlockchainParser::parse(Rule::number_and_currency, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed number and currency: {}", pair.as_str());
                }
                assert!(
                    true,
                    "Expected valid input to match the number_and_currency rule"
                );
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "100 GBP";
        match BlockchainParser::parse(Rule::number_and_currency, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(
                    true,
                    "Expected invalid input to fail the number_and_currency rule"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_entry_format() -> Result<()> {
        let valid_input = "Name: BTC; Date: 31.12.2023; Open: 234.56 USD; Close: 234.56 USD; High: 300.00 USD; Low: 200.00 USD; Volume: 500,000,000 USD;";
        match BlockchainParser::parse(Rule::entry, valid_input) {
            Ok(_) => {
                println!("Parsing succeeded for valid input");
                assert!(true, "Expected valid input to match the entry rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Name: BTC; Date: 31-12-2023; Open: 1,234.56 USD; Close: 1,234.56 USD; High: 1,300.00 USD; Low: 1,200.00 USD; Volume: 500000 USD";
        match BlockchainParser::parse(Rule::entry, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to not match the entry rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_name() -> Result<()> {
        let valid_input = "Name: BTC;";
        match BlockchainParser::parse(Rule::name, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    if let Some(inner_pair) = pair.into_inner().next() {
                        println!("Parsed name: {}", inner_pair.as_str());
                    }
                }
                assert!(true, "Expected valid input to match the name rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Name: DOGE;";
        match BlockchainParser::parse(Rule::name, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the name rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_date_entry() -> Result<()> {
        let valid_input_1 = "Date: 25.12.2023;";
        match BlockchainParser::parse(Rule::date_entry, valid_input_1) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed date entry (format 1): {}", pair.as_str());
                }
                assert!(
                    true,
                    "Expected valid input to match the date_entry rule (format 1)"
                );
            }
            Err(e) => {
                return Err(anyhow!(
                    "Parsing failed for valid input (format 1): {:?}",
                    e
                ));
            }
        }

        let valid_input_2 = "Date: 12/25/2023;";
        match BlockchainParser::parse(Rule::date_entry, valid_input_2) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed date entry (format 2): {}", pair.as_str());
                }
                assert!(
                    true,
                    "Expected valid input to match the date_entry rule (format 2)"
                );
            }
            Err(e) => {
                return Err(anyhow!(
                    "Parsing failed for valid input (format 2): {:?}",
                    e
                ));
            }
        }

        let invalid_input = "Date: 25-12-23;";
        match BlockchainParser::parse(Rule::date_entry, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the date_entry rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_open() -> Result<()> {
        let valid_input = "Open: 200.34 USD;";
        match BlockchainParser::parse(Rule::open, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    if let Some(inner_pair) = pair.into_inner().next() {
                        println!("Parsed open: {}", inner_pair.as_str());
                    }
                }
                assert!(true, "Expected valid input to match the open rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Open: two hundred USD;";
        match BlockchainParser::parse(Rule::open, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the open rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_close() -> Result<()> {
        let valid_input = "Close: 200.34 USD;";
        match BlockchainParser::parse(Rule::close, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    if let Some(inner_pair) = pair.into_inner().next() {
                        println!("Parsed close: {}", inner_pair.as_str());
                    }
                }
                assert!(true, "Expected valid input to match the close rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Close: two hundred USD;";
        match BlockchainParser::parse(Rule::close, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the close rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_high() -> Result<()> {
        let valid_input = "High: 30000 USD;";
        match BlockchainParser::parse(Rule::high, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed high value: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the high rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "High: thirty thousand USD;";
        match BlockchainParser::parse(Rule::high, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the high rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_low() -> Result<()> {
        let valid_input = "Low: 30000 USD;";
        match BlockchainParser::parse(Rule::low, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed low value: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the low rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Low: thirty thousand USD;";
        match BlockchainParser::parse(Rule::low, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the low rule");
            }
        }

        Ok(())
    }

    #[test]
    fn test_volume() -> Result<()> {
        let valid_input = "Volume: 9,822,932,731 USD;";
        match BlockchainParser::parse(Rule::volume, valid_input) {
            Ok(mut parsed) => {
                if let Some(pair) = parsed.next() {
                    println!("Parsed volume value: {}", pair.as_str());
                }
                assert!(true, "Expected valid input to match the volume rule");
            }
            Err(e) => {
                return Err(anyhow!("Parsing failed for valid input: {:?}", e));
            }
        }

        let invalid_input = "Volume: five hundred thousand USD;";
        match BlockchainParser::parse(Rule::volume, invalid_input) {
            Ok(_) => {
                return Err(anyhow!("Parsing succeeded unexpectedly for invalid input"));
            }
            Err(e) => {
                println!("Parsing failed as expected for invalid input: {:?}", e);
                assert!(true, "Expected invalid input to fail the volume rule");
            }
        }

        Ok(())
    }
}
