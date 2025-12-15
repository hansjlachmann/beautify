use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: beautify <json-string>");
        eprintln!("Example: beautify '{{\"name\":\"John\",\"age\":30}}'");
        process::exit(1);
    }

    let json_input = &args[1];

    match serde_json::from_str::<serde_json::Value>(json_input) {
        Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
            Ok(beautified) => println!("{}", beautified),
            Err(e) => {
                eprintln!("Error beautifying JSON: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            process::exit(1);
        }
    }
}
