use regex::Regex;

use clap::{
    // Args,
    Parser,
    Subcommand,
    ValueEnum,
};
use clap::error::{ Error, ErrorKind };

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct CmdAppArgs {
    /// The entity argument
    #[clap(subcommand)]
    pub command_type: CommandType,
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// Collect data from external API to local files
    Collect(CollectArgs),

    /// Load local data into the database
    Load(LoadArgs),
}


// ----- Common -----

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum SourceType {
    Nhl,
    SportRadar,
}

// Validate season single or range formats
fn validate_season(s: &str) -> Result<String, Error> {
    let re = Regex::new(r"^\d{8}|\d{8}:\d{8}$").unwrap();
    if re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err(Error::raw(
            ErrorKind::ValueValidation,
            "season must be format YYY1YYY2 or YYY1YYY2:YYY3YYY4"
        ))
    }
}


// ----- Collect -----

#[derive(Parser, Debug)]
pub struct CollectArgs {
    /// The external API source
    #[arg(long, value_enum)]
    pub source: SourceType,

    // The data entity to collect
    #[arg(long, value_enum)]
    pub entity: CollectEntityType,

    // Season of entity to collect
    #[arg(long, value_parser = validate_season)]
    pub season: Option<String>,

    // Team of entity to collect
    #[arg(long)]
    pub team: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum CollectEntityType {
    TeamImage,
    TeamSeason,
    Roster,
    Player,
}

impl CollectArgs {
    // Post-parse validation
    pub fn validate(self) -> Result<Self, Error> {
        let season_entities = [
            CollectEntityType::TeamImage, 
            CollectEntityType::TeamSeason, 
            CollectEntityType::Roster,
        ];
        if season_entities.contains(&self.entity) && self.season.is_none() {
            Err(Error::raw(
                ErrorKind::MissingRequiredArgument,
                format!("entity {:?} requires a season argument\n", self.entity)
            ))
        } else {
             Ok(self)
        }
    }
}


// ----- Load -----

#[derive(Parser, Debug)]
pub struct LoadArgs {
    /// The source local data
    #[arg(long, value_enum)]
    pub source: SourceType,

    // The data entity to load
    #[arg(long, value_enum)]
    pub entity: LoadEntityType,

    // Season of entity to load
    #[arg(long, value_parser = validate_season)]
    pub season: Option<String>,

    // Team of entity to load
    #[arg(long)]
    pub team: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum LoadEntityType {
    League,
    Season,
    Team,
    Roster,
    Player,
}

impl LoadArgs {
    // Post-parse validation
    pub fn validate(self) -> Result<Self, Error> {
        let season_entities = [
            LoadEntityType::Team, 
            LoadEntityType::Roster,
        ];
        if season_entities.contains(&self.entity) && self.season.is_none() {
            Err(Error::raw(
                ErrorKind::MissingRequiredArgument,
                format!("entity {:?} requires a season argument\n", self.entity)
            ))
        } else {
             Ok(self)
        }
    }
}
