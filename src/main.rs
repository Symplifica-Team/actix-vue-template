use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "index.html")]
struct Index {
    items: String
}

#[get("/")]
async fn index() -> impl Responder {
    let items = vec!["Apple", "Banana", "Cherry"];
    let items_json = serde_json::to_string(&items).unwrap();
    let view_template = Index { items: items_json }.render_once().unwrap();
    HttpResponse::Ok().body(view_template)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}