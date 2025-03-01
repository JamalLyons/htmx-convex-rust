mod convex_types;

use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Utc};
use convex::{ConvexClient, Value as ConvexValue};
use convex_typegen::convex::{ConvexClientExt, ConvexValueExt};
use convex_types::{CompleteQuizArgs, GetPlayerScoreArgs, ListArgs, QuizTable, ResetQuizArgs};
use env_logger;
use lazy_static::lazy_static;
use log::{debug, error, info};
use rand::seq::IndexedRandom;
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
    pub static ref CONVEX_URL: String = "https://rapid-labrador-387.convex.cloud".to_string();
}

struct AppState {
    db: Mutex<ConvexClient>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    info!("Starting server at http://localhost:8080");

    let convex_client = ConvexClient::new(&CONVEX_URL).await.unwrap();
    let state = web::Data::new(AppState {
        db: Mutex::new(convex_client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index_endpoint)
            .service(dashboard_endpoint)
            .service(start_quiz_endpoint)
            .service(quiz_endpoint)
            .service(submit_answer)
            .service(reset_quiz_endpoint)
    })
    .workers(1)
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
async fn dashboard_endpoint(data: web::Data<AppState>) -> impl Responder {
    let quizzes = match get_quizzes(&data).await {
        Ok(q) => q,
        Err(e) => {
            error!("Failed to fetch quizzes: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish();
        }
    };

    let player_score = match get_player_score(&data).await {
        Ok(score) => score,
        Err(e) => {
            error!("Failed to fetch player score: {}", e);
            0.0
        }
    };

    let mut context = tera::Context::new();
    context.insert("current_year", &current_year());
    context.insert("quizzes", &quizzes);
    context.insert("player_score", &player_score);

    let page_content = TEMPLATES.render("dashboard.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/start-quiz")]
async fn start_quiz_endpoint(data: web::Data<AppState>) -> impl Responder {
    let quizzes = match get_quizzes(&data).await {
        Ok(q) => {
            // debug!("Start quiz - Fetched quizzes: {:#?}", q);
            q
        }
        Err(e) => {
            error!("Failed to fetch quizzes: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish();
        }
    };

    // Get a random incomplete quiz
    let incomplete_quizzes: Vec<&QuizTable> =
        quizzes.iter().filter(|quiz| !quiz.complete).collect();

    debug!("Found {} incomplete quizzes", incomplete_quizzes.len());

    match incomplete_quizzes.choose(&mut rand::rng()) {
        Some(quiz) => {
            info!("Starting quiz: {}", quiz.name);
            HttpResponse::Found()
                .append_header(("Location", format!("/quiz/{}", quiz._id)))
                .finish()
        }
        None => {
            info!("No incomplete quizzes found");
            HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish()
        }
    }
}

#[get("/quiz/{id}")]
async fn quiz_endpoint(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let quizzes = match get_quizzes(&data).await {
        Ok(q) => q,
        Err(e) => {
            error!("Failed to fetch quizzes: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish();
        }
    };

    let quiz_id = path.into_inner();
    debug!("Quiz ID: {}", quiz_id);

    if let Some(quiz) = quizzes.iter().find(|q| q._id == quiz_id) {
        if quiz.questions.is_empty() {
            info!("Quiz {} has no questions", quiz_id);
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
        error!("Quiz not found: {}", quiz_id);
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
    current_score: u8,
}

#[post("/submit-answer")]
async fn submit_answer(
    form: web::Form<AnswerSubmission>,
    data: web::Data<AppState>,
) -> impl Responder {
    let quizzes = match get_quizzes(&data).await {
        Ok(q) => q,
        Err(e) => {
            error!("Failed to fetch quizzes: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/dashboard"))
                .finish();
        }
    };

    if let Some(quiz) = quizzes.iter().find(|q| q.name == form.quiz_id) {
        let current_question = &quiz.questions[form.question_index];
        let is_correct = form.answer == current_question.correct_answer as usize;
        let next_question_index = form.question_index + 1;
        let total_questions = quiz.questions.len();

        let points_per_question = (quiz.points / total_questions as f64) as u8;
        let new_score = if is_correct {
            form.current_score + points_per_question
        } else {
            form.current_score
        };

        info!(
            "Quiz: {}, Question: {}/{}, Correct: {}, Score: {}",
            quiz.name, next_question_index, total_questions, is_correct, new_score
        );

        let mut context = tera::Context::new();
        context.insert("quiz", quiz);
        context.insert("current_question", &next_question_index);
        context.insert("total_questions", &total_questions);
        context.insert("is_correct", &is_correct);
        context.insert("current_score", &new_score);

        if next_question_index >= total_questions {
            // Calculate percentage score
            let percentage = (new_score as f64 / quiz.points as f64) * 100.0;
            let passed = percentage >= 70.0;

            context.insert("percentage", &percentage);
            context.insert("passed", &passed);

            // If passed, update the quiz status in Convex
            if passed {
                if let Err(e) = mark_quiz_complete(&data, &quiz._id).await {
                    error!("Failed to mark quiz as complete: {}", e);
                }
            }

            let page_content = TEMPLATES.render("quiz_complete.html", &context).unwrap();
            HttpResponse::Ok().body(page_content)
        } else {
            let page_content = TEMPLATES.render("quiz_question.html", &context).unwrap();
            HttpResponse::Ok().body(page_content)
        }
    } else {
        error!("Quiz not found: {}", form.quiz_id);
        HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish()
    }
}

#[post("/reset-quiz")]
async fn reset_quiz_endpoint(
    form: web::Form<ResetQuizForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = un_mark_quiz_complete(&data, &form.quiz_id).await {
        error!("Failed to reset quiz: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    // Return the updated quiz item HTML
    let quizzes = match get_quizzes(&data).await {
        Ok(q) => q,
        Err(e) => {
            error!("Failed to fetch quizzes: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(quiz) = quizzes.iter().find(|q| q._id == form.quiz_id) {
        let mut context = tera::Context::new();
        context.insert("quiz", quiz);
        let page_content = TEMPLATES
            .render("dashboard_quiz_item.html", &context)
            .unwrap();
        HttpResponse::Ok().body(page_content)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(serde::Deserialize)]
struct ResetQuizForm {
    quiz_id: String,
}

// Helper function to get quizzes from Convex
async fn get_quizzes(data: &web::Data<AppState>) -> Result<Vec<QuizTable>, String> {
    let results = data
        .db
        .lock()
        .unwrap()
        .query(
            ListArgs::FUNCTION_PATH,
            ConvexClient::prepare_args(ListArgs {}),
        )
        .await
        .map_err(|e| e.to_string())?;

    match results {
        convex::FunctionResult::Value(value) => {
            if let ConvexValue::Array(arr) = value {
                // debug!("Raw JSON from Convex: {:?}", arr);
                let quizzes = arr
                    .into_iter()
                    .filter_map(|v| {
                        let serde_value = v.into_serde_value();
                        // debug!("Converting value: {:?}", serde_value);
                        let json_value = serde_json::to_value(serde_value).ok()?;
                        match serde_json::from_value::<QuizTable>(json_value.clone()) {
                            Ok(quiz) => Some(quiz),
                            Err(e) => {
                                error!(
                                    "Failed to deserialize quiz: {:?}\nValue: {:?}",
                                    e, json_value
                                );
                                None
                            }
                        }
                    })
                    .collect();
                Ok(quizzes)
            } else {
                Ok(vec![])
            }
        }
        convex::FunctionResult::ErrorMessage(error) => Err(error),
        convex::FunctionResult::ConvexError(error) => Err(error.message),
    }
}

async fn mark_quiz_complete(data: &web::Data<AppState>, quiz_id: &str) -> Result<(), String> {
    data.db
        .lock()
        .unwrap()
        .mutation(
            "quiz:completeQuiz",
            ConvexClient::prepare_args(CompleteQuizArgs {
                id: quiz_id.to_string(),
            }),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn un_mark_quiz_complete(data: &web::Data<AppState>, quiz_id: &str) -> Result<(), String> {
    data.db
        .lock()
        .unwrap()
        .mutation(
            "quiz:resetQuiz",
            ConvexClient::prepare_args(ResetQuizArgs {
                id: quiz_id.to_string(),
            }),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn get_player_score(data: &web::Data<AppState>) -> Result<f64, String> {
    let result = data
        .db
        .lock()
        .unwrap()
        .query(
            GetPlayerScoreArgs::FUNCTION_PATH,
            ConvexClient::prepare_args(GetPlayerScoreArgs {}),
        )
        .await
        .map_err(|e| e.to_string())?;

    match result {
        convex::FunctionResult::Value(value) => {
            if let ConvexValue::Float64(score) = value {
                Ok(score)
            } else {
                Ok(0.0)
            }
        }
        convex::FunctionResult::ErrorMessage(error) => Err(error),
        convex::FunctionResult::ConvexError(error) => Err(error.message),
    }
}

fn current_year() -> String {
    Utc::now().year().to_string()
}
