mod args;
mod ops;

use clap::Parser;

use args::{ CmdAppArgs, CommandType, CollectArgs, LoadArgs };
use ops::collect_ops::handle_collect_command;
use ops::load_ops::handle_load_command;

fn main() {
    // Parse command-line
    let args:CmdAppArgs = CmdAppArgs::parse();

    // Invoke appropriate handler 
    match args.command_type {
        // Demonstrate collect data
        CommandType::Collect(args) => {
            match CollectArgs::validate(args) {
                Ok(valid_args) => {
                    handle_collect_command(valid_args);
                }
                Err(e) => {
                    e.print().expect("Failed to print validation error");
                    std::process::exit(1);
                }
            }
        }
        // Demonstrate load data
        CommandType::Load(args) => {
            match LoadArgs::validate(args) {
                Ok(valid_args) => {
                    handle_load_command(valid_args);
                }
                Err(e) => {
                    e.print().expect("Failed to print validation error");
                    std::process::exit(1);
                }
            }
        }
    };
}
