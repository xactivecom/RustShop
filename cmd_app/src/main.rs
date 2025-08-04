mod args;
mod ops;

use clap::Parser;

use args::EntityType;
use args::CmdAppArgs;
use ops::user_ops::handle_user_command;
use ops::user_ops::handle_video_command;

fn main() {
    // Parse command-line
    let args:CmdAppArgs = CmdAppArgs::parse();

    // Invoke appropriate handler 
    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Video(video) => handle_video_command(video),
    };
}
