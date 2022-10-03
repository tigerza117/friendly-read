#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}

async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}
