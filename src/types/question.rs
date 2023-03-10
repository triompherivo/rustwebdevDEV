use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

/*#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}*/

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);
