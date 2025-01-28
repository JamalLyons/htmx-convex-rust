use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = match Tera::new(source) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[get("/")]
async fn index_endpoint() -> impl Responder {
    let context = tera::Context::new();
    // context.insert("message_from_rust", "Hello from Rust!");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/dashboard")]
async fn dashboard_endpoint() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("dashboard.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index_endpoint)
            .service(dashboard_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
