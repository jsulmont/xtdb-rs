use serde_json::{json, Value};
use std::{env, fs, io, io::Read};
use xtdb_rs::client::{CustomError, XtdbClient, XtqlQuery};
use xtdb_rs::xtql::parse::parse_xtql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:3000";
    let client = XtdbClient::new(base_url);
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

    let query = XtqlQuery {
        query: parse_xtql(&content).unwrap(),
        options: json!({}),
    };

    match client.execute_query(query).await {
        Ok(response) => {
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
            Ok(())
        }
        Err(e) => match e {
            CustomError::XtdbError(ref err) => {
                if let Ok(parsed_json) = serde_json::from_str::<Value>(err) {
                    eprintln!(
                        "Error: {}",
                        serde_json::to_string_pretty(&parsed_json).unwrap()
                    );
                } else {
                    eprintln!("Error: {}", err); // Fallback in case the error is not valid JSON
                }
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
            _ => {
                eprintln!("An error occurred: {}", e); // Print to stderr
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        },
    }
}
