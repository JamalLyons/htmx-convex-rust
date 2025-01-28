use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Utc};
use lazy_static::lazy_static;
use serde::Serialize;
use tera::Tera;

#[derive(Debug, Clone, Serialize)]
struct Quiz {
    pub id: String,
    pub subject: String,
    pub name: String,
    pub points_awarded: u8,
    pub completed: bool,
}

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

#[get("/")]
async fn index_endpoint() -> impl Responder {
    let mut context = tera::Context::new();
    let year = current_year();
    context.insert("current_year", &year);
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/dashboard")]
async fn dashboard_endpoint() -> impl Responder {
    // Sample quiz data
    let quizzes = vec![
        Quiz {
            id: "1".to_string(),
            subject: "Programming".to_string(),
            name: "Intro to Rust".to_string(),
            points_awarded: 10,
            completed: false,
        },
        Quiz {
            id: "2".to_string(),
            subject: "Web Development".to_string(),
            name: "HTML Basics".to_string(),
            points_awarded: 5,
            completed: true,
        },
        Quiz {
            id: "3".to_string(),
            subject: "Data Science".to_string(),
            name: "Data Analysis with Python".to_string(),
            points_awarded: 15,
            completed: false,
        },
    ];

    let mut context = tera::Context::new();

    let year = current_year();
    context.insert("current_year", &year);
    context.insert("quizzes", &quizzes);

    let page_content = TEMPLATES.render("dashboard.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

fn current_year() -> String {
    Utc::now().year().to_string()
}
