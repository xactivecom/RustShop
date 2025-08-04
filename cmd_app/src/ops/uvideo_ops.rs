use crate::args::{
    VideoSubcommand,
    VideoCommand,
    CreateVideo,
    UpdateVideo,
    DeleteEntity,
};

pub fn handle_video_command(video: VideoCommand) {
    let command = video.command;
    match command {
        VideoSubcommand::Create(video) => {
            create_video(video);
        }
        VideoSubcommand::Update(video) => {
            update_video(video);
        }
        VideoSubcommand::Delete(entity) => {
            delete_video(entity);
        }
        VideoSubcommand::Show => {
            show_videos();
        }
    }
}

fn create_video(video: CreateVideo) {
    println!("create_video {:?}", video);
}

fn update_video(video: UpdateVideo) {
    println!("update_video {:?}", video);
}

fn delete_video(entity: DeleteEntity) {
    println!("delete_video {:?}", entity);
}

fn show_videos() {
    println!("show_videos");
}
