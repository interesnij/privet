#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;
mod vars;

use actix_web::{
    HttpServer,
    App,
    middleware,
};
use actix_files::Files;
use crate::routes::routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_redis::RedisSession;

    HttpServer::new(|| {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .service(_files)
            .service(_files2)
            .configure(routes)
    })
    .bind("0.0.0.0:9080")?
    //.bind("http://вселенная.рус:8000")?
    .run()
    .await
}
