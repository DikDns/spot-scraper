use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub nim: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub credits: u8,
    pub lecturer: String,
    pub academic_year: String,
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rps {
    pub id: Option<u64>,
    pub href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicInfo {
    pub id: Option<u64>,
    pub course_id: Option<u64>,
    pub access_time: Option<NaiveDateTime>,
    pub is_accessible: bool,
    pub href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailCourse {
    #[serde(flatten)]
    pub course_info: Course,
    pub description: String,
    pub rps: Rps,
    pub topics: Vec<TopicInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub id: u32,
    pub youtube_id: Option<String>,
    pub raw_html: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Pending,
    Submitted,
    Graded,
    NotSubmitted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: Option<u64>,
    pub content: String,
    pub file_href: Option<String>,
    pub is_graded: bool,
    pub lecturer_notes: String,
    pub score: f32,
    pub date_submitted: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub course_id: u64,
    pub topic_id: u64,
    pub token: String,
    pub title: String,
    pub description: String,
    pub file: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub status: TaskStatus,
    pub answer: Option<Answer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicDetail {
    pub id: u64,
    pub access_time: Option<NaiveDateTime>,
    pub is_accessible: bool,
    pub href: String,
    pub description: Option<String>,
    pub contents: Vec<Content>,
    pub tasks: Vec<Task>,
}
