use crate::args::{
    CollectArgs,
    SourceType,
};

pub fn handle_collect_command(args: CollectArgs) {

    // Demonstrate appropriate source API
    match args.source {
        SourceType::Nhl => {
            println!("collect NHL entity {:?}, season {:?}, team {:?}", args.entity, args.season, args.team);
        },
        SourceType::SportRadar => {
            println!("collect SportRadar entity {:?}, season {:?}, team {:?}", args.entity, args.season, args.team);
        },
        _ => {
            eprintln!("collect unrecognized source {:?}", args.source);
            std::process::exit(1);
        }
    }
}
