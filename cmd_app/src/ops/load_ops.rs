use crate::args::{
    LoadArgs,
    SourceType,
};

pub fn handle_load_command(args: LoadArgs) {

    // Demonstrate appropriate source API
    match args.source {
        SourceType::Nhl => {
            println!("load NHL {:?}, season {:?}, team {:?}", args.entity, args.season, args.team);
        },
        SourceType::SportRadar => {
            println!("load SportRadar {:?}, season {:?}, team {:?}", args.entity, args.season, args.team);
        },
        _ => {
            eprintln!("load unrecognized source {:?}", args.source);
            std::process::exit(1);
        }
    }
}
