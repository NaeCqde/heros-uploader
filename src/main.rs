mod catbox;
mod data;
mod env;
mod gofile;
mod handler;
mod html;
mod uploader;

use crate::handler::{handle_get_upload, handle_post_upload, index};
use ntex::web::{self, guard, types, HttpServer};
use std::{error::Error, process::exit};
use tokio::{signal, task::JoinSet};

#[ntex::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let env = env::load().expect("load env failure");

    let mut set = JoinSet::new();
    let server = HttpServer::new(|| {
        web::App::new()
            .state(
                // change json extractor configuration
                types::JsonConfig::default(),
            )
            .route("/", web::get().to(index))
            .route("/upload", web::get().to(handle_get_upload))
            .route(
                "/upload",
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(handle_post_upload),
            )
    })
    .workers(env.workers)
    .bind((env.host.to_owned(), env.port))
    .expect("failed to bind port");
    set.spawn(server.run());

    loop {
        tokio::select! {

            _ = signal::ctrl_c()=>{
                set.shutdown().await;

                exit(0);
            },
        }
    }
}
