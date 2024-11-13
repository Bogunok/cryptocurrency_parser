use anyhow::Result;
use cryptocurrency_parser::parse_file_to_json;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() != 4 {
                eprintln!("Error: 'parse' command requires an input file and an output file.");
                return Ok(());
            }
            let input_file_name = &args[2];
            let output_file_name = &args[3];

            let input_path = std::path::Path::new(input_file_name);
            let output_path = std::path::Path::new(output_file_name);

            match parse_file_to_json(input_path, output_path) {
                Ok(_) => println!("JSON data has been written to {}", output_file_name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "help" => {
            print_help();
        }
        "credits" => {
            print_credits();
        }
        _ => {
            eprintln!("Unknown command");
            println!("Run 'cargo run help' to see available commands.");
        }
    }
    Ok(())
}

fn print_help() {
    println!("Usage: run code using specific arguments\n");
    println!("cargo run <command> [options]\n");
    println!("Commands:");
    println!("  parse <input_file> <output_file>  - Parse the input file and save to output file");
    println!("  help                              - Show help information");
    println!("  credits                           - Show credits");
    println!("\nAdditional explanations:\n");
    println!("To use this parser you must have data in a specific format, such as in data.txt");
    println!("Example structure:");
    println!("Name: SOL; Date: 08.11.2024; Open: 196.34 USD; Close: 203.05 USD; High: 204.76 USD; Low: 202.22 USD; Volume: 5,668,932,608 USD;");
    println!("Write all entries in separate lines in the input file.");
    println!("You will get a well-formatted JSON output in the specified output file.");
}

fn print_credits() {
    println!("Cryptocurrency Parser");
    println!("Developed by Yelyzaveta Bohun");
}
