extern crate core;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::{
    http::header, middleware::Logger, web, App, HttpServer,
};
use dotenv::dotenv;
use url::Url;

mod api;
mod client;
mod model;
mod puller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let forward_url = format!("https://img.niceoppai.net");
    let forward_url = Url::parse(forward_url.as_str()).unwrap();

    log::info!("forwarding to {forward_url}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(forward_url.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(api::list)
            .service(web::scope("/view").service(api::view))
            .service(
                web::scope("/proxy").default_service(web::to(api::forward)), //.route("/index.html", web::get().to(index)),
            )
    })
    .bind(("127.0.0.1", 8844))?
    .workers(2)
    .run()
    .await
}
