use std::io;
use converters::*;

mod converters;

struct ParsedLine {
    value: u32,
    from: char,
    target: char,
}

fn main() {
    println!("Easy to use temperature converter.\nExample: '10C to F', '300K to C'.");

    let mut buf = String::new();

    loop {
        // Wait for use input
        let _ = match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error while reading input: {}", err);
                return;
            }
        };

        let parsed_line = match parse(&buf) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Parsing error: {}", err);
                return;
            }
        };

        // Clear buffer
        buf = String::new();

        // Calculate to target type
        let result = match parsed_line.from {
            'C' =>
                match parsed_line.target {
                    'F' => celsius::to_fahrenheit(parsed_line.value),
                    'K' => celsius::to_kelvins(parsed_line.value),
                    _ => {
                        return;
                    }
                }
            'F' =>
                match parsed_line.target {
                    'C' => fahrenheit::to_celsius(parsed_line.value),
                    'K' => fahrenheit::to_kelvins(parsed_line.value),
                    _ => {
                        return;
                    }
                }
            'K' =>
                match parsed_line.target {
                    'C' => kelvins::to_celsius(parsed_line.value),
                    'F' => kelvins::to_fahrenheit(parsed_line.value),
                    _ => {
                        return;
                    }
                }
            _ => {
                return;
            }
        };

        // Print result
        println!(
            "Converting {} from {} to {}... is equal {}. To quit press [Ctrl + C].",
            parsed_line.value,
            parsed_line.from,
            parsed_line.target,
            result
        );
    }
}

fn parse(input: &String) -> Result<ParsedLine, &str> {
    let words = input.split_whitespace().collect::<Vec<&str>>();

    // User must input three arguments
    if words.len() != 3 {
        return Err("Error while parsing input, wrong arguments count.");
    }

    // Read first argument and split it in chars
    let first_arg = words[0].chars().collect::<Vec<char>>();

    // All chars must be numeric exсept last one
    let (value, from) = first_arg.split_at(first_arg.len() - 1);

    // Convert value to u32
    let value = match value.iter().collect::<String>().parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            return Err("Error while converting value to digit.");
        }
    };

    // Shadowing useless slice
    let from = from[0];

    if !from.is_alphabetic() {
        return Err("First argument last char must be alphabetic: C, F or K.");
    }

    // Get target type from last argument
    let target = match words[2].chars().last() {
        Some(val) => val,
        None => {
            return Err("You must specify converter target type.");
        }
    };

    if !target.is_alphabetic() {
        return Err("You must specify converter target type.");
    }

    Ok(ParsedLine { value, from, target })
}
