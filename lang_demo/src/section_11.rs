///////////////////////////////
// Course Section 11
///////////////////////////////

use std::env;
use std::fs;

use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    // Write stderr
    eprintln!("{} - replace string with a new string", "find and replace".green());
    eprintln!("Usage: target_string replacement_string input_file output_file");
}

fn parse_args() -> Arguments {
    // Get command-line arguments from environment, skip program name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!("{} Usage: target_string replacement_string input_file output_file", "Error:".red().bold());
        std::process::exit(1);
    }

    // Collect arguments
    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{} - Failed to read from file {} {:?}", 
                "Error".red().bold(), &args.input_file, error);
            std::process::exit(1);
        }
    };

    println!("Data: {}", data);
    let replaced_data = match replace_string(&args.pattern, &args.replace, &data) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("{} - Failed to replace data {:?}",
                "Error".red().bold(), error);
            std::process::exit(1);
        }
    };
    println!("Replaced data: {}", replaced_data);

    match fs::write(&args.output_file, replaced_data) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{} - Failed to write to file {} {:?}", 
                "Error".red().bold(), &args.input_file, error);
            std::process::exit(1);
        }
    }
}

fn replace_string(target: &str, replace: &str, data: &str) -> Result<String, regex::Error> {
    let reg = Regex::new(target)?;
    Ok(reg.replace_all(data, replace).to_string())
}

pub fn run_lesson() {
    println!("\nSection 11:");

    // let args = parse_args();
    // println!("{:?}", args);

    // read_and_write(&args);
}
