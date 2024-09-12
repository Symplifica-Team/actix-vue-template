use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "index.html")]
struct Index {
    items: Vec<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    let ctx = Index {
        items: vec![String::from("Manzana"), String::from("Pera")],
    };
    HttpResponse::Ok().body(ctx.render_once().unwrap())
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