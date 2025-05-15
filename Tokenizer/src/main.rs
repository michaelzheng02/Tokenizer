mod exp_parser;

use exp_parser::{ExprParser, Value};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut variables: HashMap<String, Value> = HashMap::new();
    println!("Enter your program or type exit to quit: ");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let input = line.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if input.is_empty() {
            continue;
        }

        if !input.ends_with(';') {
            eprintln!("error: program must end with a semicolon ;");
            continue;
        }

        let statements: Vec<&str> = input
            .split(';')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        for statement in statements {
            let parts: Vec<&str> = statement.split('=').map(str::trim).collect();
            if parts.len() != 2 {
                eprintln!(
                    "error: invalid assignment format. Use the following format `name = value;`"
                );
                continue;
            }

            let var_name = parts[0];
            if !is_valid_identifier(var_name) {
                eprintln!("error: invalid identifier '{}'", var_name);
                continue;
            }

            let expr = parts[1];
            let mut parser = ExprParser::new(expr, &variables);

            match parser.parse() {
                Ok(value) => {
                    variables.insert(var_name.to_string(), value);
                }
                Err(err) => {
                    eprintln!("error: {}", err);
                    continue;
                }
            }
        }

        for (name, value) in &variables {
            match value {
                Value::Int(n) => println!("{} = {}", name, n),
                Value::Str(s) => println!("{} = \"{}\"", name, s),
            }
        }
    }
}

fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}
