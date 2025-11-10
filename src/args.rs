use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author , version , about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {

    User(UserCommand),

    Video(VideoCommand),

    View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),

    Update(UpdateUser),

    Delete(DeleteEntity),

    Show,
}
#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,

}

#[derive(Debug, Args)]
pub struct UpdateUser {
    pub id: i32,
    pub name: String, 
    pub email: String,
}


#[derive(Debug, Args)]
pub struct DeleteEntity {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug,Subcommand)]
pub enum VideoSubcommand {
    Create(CreateVideo),

    Update(UpdateVideo),

    Delete(DeleteEntity),

    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    pub title: String,

    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    pub id: i32,
    pub title: String, 
    pub description: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    Create(CreateView),

    Show,

    ShowPretty
}

#[derive(Debug, Args)]
pub struct CreateView {
    pub user_id: i32,
    pub video_id: i32,
    pub watch_start: chrono::NaiveDateTime,
    pub duration: i32,
}