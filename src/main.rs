use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Utc};
use lazy_static::lazy_static;
use rand::seq::IndexedRandom;
use serde::Serialize;
use tera::Tera;

#[derive(Debug, Clone, Serialize)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize, // Index of the correct answer (0-based)
}

#[derive(Debug, Clone, Serialize)]
pub struct Quiz {
    pub id: String,
    pub subject: String,
    pub name: String,
    pub description: String,
    pub points_awarded: u8,
    pub completed: bool,
    pub questions: Vec<Question>,
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
    pub static ref QUIZZES: Vec<Quiz> = vec![
        Quiz {
            id: "1".to_string(),
            subject: "Rust Fundamentals".to_string(),
            name: "Ownership and Borrowing".to_string(),
            points_awarded: 15,
            completed: false,
            description: "Test your knowledge of Rust's ownership system and borrowing rules."
                .to_string(),
            questions: vec![
                Question {
                    text: "What is the primary benefit of Rust's ownership system?".to_string(),
                    options: vec![
                        "Memory safety without garbage collection".to_string(),
                        "Automatic memory management with GC".to_string(),
                        "Dynamic typing capabilities".to_string(),
                        "Interpreted language features".to_string(),
                    ],
                    correct_answer: 0,
                },
                Question {
                    text: "In Rust, how many owners can a piece of data have at a time?"
                        .to_string(),
                    options: vec![
                        "As many as needed".to_string(),
                        "Two".to_string(),
                        "One".to_string(),
                        "Zero".to_string(),
                    ],
                    correct_answer: 2,
                },
                Question {
                    text: "What happens when a variable goes out of scope in Rust?".to_string(),
                    options: vec![
                        "Nothing".to_string(),
                        "The memory is automatically freed".to_string(),
                        "It becomes null".to_string(),
                        "It's garbage collected".to_string(),
                    ],
                    correct_answer: 1,
                },
                Question {
                    text: "Which of these is a mutable borrow?".to_string(),
                    options: vec![
                        "&self".to_string(),
                        "&'static str".to_string(),
                        "&mut String".to_string(),
                        "String".to_string(),
                    ],
                    correct_answer: 2,
                },
                Question {
                    text: "What is the purpose of the 'move' keyword in Rust?".to_string(),
                    options: vec![
                        "To copy data".to_string(),
                        "To force ownership transfer".to_string(),
                        "To create a reference".to_string(),
                        "To delete data".to_string(),
                    ],
                    correct_answer: 1,
                },
            ],
        },
        Quiz {
            id: "2".to_string(),
            subject: "Web Development".to_string(),
            name: "HTMX Basics".to_string(),
            points_awarded: 10,
            completed: false,
            description: "Learn about HTMX attributes and basic Ajax patterns.".to_string(),
            questions: vec![],
        },
        Quiz {
            id: "3".to_string(),
            subject: "Database".to_string(),
            name: "Convex Fundamentals".to_string(),
            points_awarded: 12,
            completed: false,
            description: "Explore the basics of Convex database and its features.".to_string(),
            questions: vec![],
        },
        Quiz {
            id: "4".to_string(),
            subject: "Rust Web".to_string(),
            name: "Actix-web Essentials".to_string(),
            points_awarded: 20,
            completed: false,
            description: "Master the fundamentals of the Actix-web framework.".to_string(),
            questions: vec![],
        },
        Quiz {
            id: "5".to_string(),
            subject: "Frontend".to_string(),
            name: "Tailwind CSS Mastery".to_string(),
            points_awarded: 8,
            completed: false,
            description: "Learn essential Tailwind CSS utilities and patterns.".to_string(),
            questions: vec![],
        },
    ];
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index_endpoint)
            .service(dashboard_endpoint)
            .service(start_quiz_endpoint)
            .service(quiz_endpoint)
            .service(submit_answer)
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
    let mut context = tera::Context::new();
    let year = current_year();
    context.insert("current_year", &year);
    context.insert("quizzes", &*QUIZZES);
    let page_content = TEMPLATES.render("dashboard.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/start-quiz")]
async fn start_quiz_endpoint() -> impl Responder {
    // Get a random incomplete quiz
    let incomplete_quizzes: Vec<&Quiz> = QUIZZES.iter().filter(|quiz| !quiz.completed).collect();

    match incomplete_quizzes.choose(&mut rand::rng()) {
        Some(quiz) => {
            // For now, redirect to dashboard with a future quiz page
            HttpResponse::Found()
                .append_header(("Location", format!("/quiz/{}", quiz.id)))
                .finish()
        }
        None => {
            // If no incomplete quizzes, redirect to dashboard
            HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish()
        }
    }
}

#[get("/quiz/{id}")]
async fn quiz_endpoint(path: web::Path<String>) -> impl Responder {
    let quiz_id = path.into_inner();

    if let Some(quiz) = QUIZZES.iter().find(|q| q.id == quiz_id) {
        if quiz.questions.is_empty() {
            return HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish();
        }

        let mut context = tera::Context::new();
        context.insert("current_year", &current_year());
        context.insert("quiz", quiz);
        context.insert("current_question", &0);
        context.insert("total_questions", &quiz.questions.len());
        context.insert("current_score", &0);

        let page_content = TEMPLATES.render("quiz.html", &context).unwrap();
        HttpResponse::Ok().body(page_content)
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish()
    }
}

#[derive(serde::Deserialize)]
struct AnswerSubmission {
    quiz_id: String,
    question_index: usize,
    answer: usize,
}

#[post("/submit-answer")]
async fn submit_answer(form: web::Form<AnswerSubmission>) -> impl Responder {
    if let Some(quiz) = QUIZZES.iter().find(|q| q.id == form.quiz_id) {
        let current_question = &quiz.questions[form.question_index];
        let is_correct = form.answer == current_question.correct_answer;
        let next_question_index = form.question_index + 1;
        let total_questions = quiz.questions.len();
        let current_score = if is_correct {
            quiz.points_awarded / total_questions as u8
        } else {
            0
        };

        let mut context = tera::Context::new();
        context.insert("quiz", quiz);
        context.insert("current_question", &next_question_index);
        context.insert("total_questions", &total_questions);
        context.insert("is_correct", &is_correct);
        context.insert("current_score", &current_score);

        // If this was the last question, show results
        if next_question_index >= total_questions {
            let page_content = TEMPLATES.render("quiz_complete.html", &context).unwrap();
            HttpResponse::Ok().body(page_content)
        } else {
            let page_content = TEMPLATES.render("quiz_question.html", &context).unwrap();
            HttpResponse::Ok().body(page_content)
        }
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish()
    }
}

fn current_year() -> String {
    Utc::now().year().to_string()
}
