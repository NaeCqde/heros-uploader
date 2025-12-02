use crate::{
    catbox::catbox,
    data::{MyError, Video},
    gofile::gofile,
};

pub async fn upload(video: Video) -> Result<Video, MyError> {
    let mut video = video;
    if let Some(thumbnail) = video.thumbnail {
        video.thumbnail = Some(catbox(&thumbnail).await?);
    }
    if let Some(src) = video.src {
        video.src = Some(gofile(&src).await?);
    }

    return Ok(video);
}
