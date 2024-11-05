#[cfg(test)]
mod tests {
    use pest::Parser;
    use solana_price_parser::SolanaParser;
    use solana_price_parser::Rule;

    #[test]
    fn test_price_value_parsing() {
        let valid_prices = vec!["123", "123.45", ".45", "0.0", "1000000"];

        for price in valid_prices {
            let parse_result = SolanaParser::parse(Rule::price_value, price);
            assert!(parse_result.is_ok(), "Failed to parse valid price: {}", price);
        }

        let invalid_prices = vec!["abc", "123.", "12.34.56", ""];

        for price in invalid_prices {
            let parse_result = SolanaParser::parse(Rule::price_value, price);
            assert!(parse_result.is_err(), "Parsed invalid price as valid: {}", price);
        }
    }
}
