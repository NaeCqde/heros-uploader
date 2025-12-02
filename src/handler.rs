use crate::{
    data::{MyError, Video},
    uploader::upload,
};
use ntex::{
    http::Response,
    web::{self, types},
};

pub async fn handle_get_upload(payload: types::Query<Video>) -> Response {
    return handle_upload(payload.0).await;
}
pub async fn handle_post_upload(payload: types::Json<Video>) -> Response {
    return handle_upload(payload.0).await;
}

pub async fn handle_upload(payload: Video) -> Response {
    match upload(payload).await {
        Ok(video) => return Response::Ok().json(&video),
        Err(e) => {
            if matches!(e, MyError::ResourceNotAccessible) {
                return Response::BadRequest()
                    .body("{\"message\":\"The provided urls are not accessible\"}");
            } else {
                log::error!(target:"uploader", "{}", e);
                return Response::InternalServerError()
                    .body("{\"message\":\"Internal server error\"}");
            }
        }
    }
}

pub async fn index() -> impl web::Responder {
    return "hello, TODO:\nWeb UI";
}
