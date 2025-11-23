use crate::{
    catbox::catbox,
    data::{MyError, Video},
    gofile::gofile,
};

pub async fn upload(video: Video) -> Result<Video, MyError> {
    let mut video = video;
    video.thumbnail = catbox(&video.thumbnail).await?;
    video.src = gofile(&video.src).await?;

    return Ok(video);
}
