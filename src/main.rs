extern crate core;
extern crate dotenv;

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::{
    get, http::header, middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use dotenv::dotenv;
use log::log;
use serde::de::Unexpected::Str;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::MangaEp;

mod client;
mod model;
mod puller;

static BASE_URL: &str = "https://www.niceoppai.net";
static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.61 Safari/537.36 RuxitSynthetic/1.0 v8007188160821992392 t5908864234688580170 athf552e454 altpriv cvcv=2 smf=0";

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
            .service(list)
            .service(web::scope("/view").service(view))
            .service(
                web::scope("/proxy").default_service(web::to(forward)), //.route("/index.html", web::get().to(index)),
            )
    })
    .bind(("127.0.0.1", 8844))?
    .workers(2)
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct Res {
    manga: model::Manga,
    pages: Vec<model::Page>,
}

#[get("")]
async fn view(query: web::Query<HashMap<String, String>>) -> impl Responder {
    match query.get("manga") {
        Some(manga_name) => {
            let link = [BASE_URL, manga_name].join("/");
            match puller::get_manga_info(link.as_str()).await {
                Some(mut manga_info) => {
                    let mut res = Res {
                        manga: manga_info.to_owned(),
                        pages: vec![],
                    };

                    if manga_info.eps.is_empty() {
                        let json = serde_json::to_string(&res).unwrap();

                        return HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .body(json);
                    }
                    let mut ep = &mut MangaEp {
                        name: "".to_string(),
                        ep_path: "".to_string(),
                    };
                    match query.get("ep") {
                        Some(var) => {
                            let index = manga_info
                                .eps
                                .iter()
                                .position(|e| e.ep_path == String::from(var));
                            match index {
                                Some(index) => {
                                    ep = &mut manga_info.eps[index];
                                }
                                None => {
                                    ep = &mut manga_info.eps[0];
                                }
                            }
                        }
                        _ => ep = &mut manga_info.eps[0],
                    }

                    let page_list =
                        puller::get_pages([link.as_str(), ep.ep_path.as_str()].join("/").as_str())
                            .await;

                    res.pages = page_list;

                    let json = serde_json::to_string(&res).unwrap();

                    HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body(json)
                }
                _ => HttpResponse::BadRequest().finish(),
            }
        }
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[get("/list")]
async fn list(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let mut num_page: i64 = 1;
    match query.get("page") {
        Some(var) => {
            num_page = var.as_str().parse::<i64>().unwrap();
        }
        _ => {}
    }

    let manga_list = puller::get_manga_list(num_page).await;
    let json = serde_json::to_string(&manga_list).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}

async fn forward(
    req: HttpRequest,
    url: web::Data<Url>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.get_ref().clone();
    match query.get("prefix") {
        Some(var) => {
            if !var.as_str().is_empty() {
                new_url =
                    Url::parse(new_url.as_str().replace("img", var.as_str()).as_str()).unwrap()
            }
        }
        _ => {}
    }
    let path = req.uri().path().split("/").collect::<Vec<&str>>()[2..].join("/");
    new_url.set_path(path.as_str());
    new_url.set_query(req.uri().query());

    let res = reqwest::Client::builder()
        .build()
        .ok()
        .expect("Fail to create client")
        .request(req.method().clone(), new_url.as_str())
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .await
        .ok()
        .expect("Fail to send");

    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }
    client_resp.insert_header((header::USER_AGENT, USER_AGENT));
    client_resp.insert_header(
        (header::CacheControl(vec![
            header::CacheDirective::Private,
            header::CacheDirective::MaxAge(360u32),
        ])),
    );

    Ok(client_resp.streaming(res.bytes_stream()))
}
