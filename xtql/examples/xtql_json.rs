// This example demonstrates how to use the parser to parse a simple XTQL query and print the result as JSON.
use std::io::Read;
use std::{env, fs, io};
use xtql::parse_xtql;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let content = if args.len() == 2 {
        let filepath = &args[1];
        fs::read_to_string(filepath).expect("Failed to read file")
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    let result = parse_xtql(&content).unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    Ok(())
}
