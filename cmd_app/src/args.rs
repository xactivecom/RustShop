use clap::{
    Args,
    Parser,
    Subcommand,
};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct CmdAppArgs {
    /// The entity argument
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Subcommand, Debug)]
pub enum EntityType {
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show videos
    Video(VideoCommand),
}

#[derive(Args, Debug)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum UserSubcommand {
    /// Create a user
    Create(CreateUser),

    /// Update a user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The id of the video to update
    pub id: i32,

    /// The title of the video
    pub title: String,

    /// The description of the video
    pub description: String,
}
