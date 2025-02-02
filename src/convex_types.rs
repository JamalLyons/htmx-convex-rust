// This file is generated by convex-typegen. Do not modify directly.
// You can find more information about convex-typegen at https://github.com/JamalLyons/convex-typegen

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizTable {
    pub _id: String,
    pub subject: String,
    pub name: String,
    pub desc: String,
    pub points: f64,
    pub complete: bool,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArgs {
    pub quizID: String,
}

impl GetArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:get";
}

impl From<GetArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: GetArgs) -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "quizID".to_string(),
            serde_json::to_value(_args.quizID).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveArgs {
    pub quizID: String,
}

impl RemoveArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:remove";
}

impl From<RemoveArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: RemoveArgs) -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "quizID".to_string(),
            serde_json::to_value(_args.quizID).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkCompleteArgs {
    pub quizID: String,
}

impl MarkCompleteArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:markComplete";
}

impl From<MarkCompleteArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: MarkCompleteArgs) -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "quizID".to_string(),
            serde_json::to_value(_args.quizID).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnMarkCompleteArgs {
    pub quizID: String,
}

impl UnMarkCompleteArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:unMarkComplete";
}

impl From<UnMarkCompleteArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: UnMarkCompleteArgs) -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "quizID".to_string(),
            serde_json::to_value(_args.quizID).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListArgs {}

impl ListArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:list";
}

impl From<ListArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: ListArgs) -> Self {
        std::collections::BTreeMap::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedQuizTableArgs {
    pub agree: Option<bool>,
}

impl SeedQuizTableArgs {
    pub const FUNCTION_PATH: &'static str = "quiz:seedQuizTable";
}

impl From<SeedQuizTableArgs> for std::collections::BTreeMap<String, serde_json::Value> {
    fn from(_args: SeedQuizTableArgs) -> Self {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "agree".to_string(),
            serde_json::to_value(_args.agree).unwrap(),
        );
        map
    }
}
