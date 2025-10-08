use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub nim: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: String,
    pub code: String,
    pub name: String,
    pub credits: u8,
    pub lecturer: String,
    pub academic_year: String,
    pub href: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rps {
    pub id: Option<String>,
    pub href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicInfo {
    pub id: Option<String>,
    pub access_time: Option<String>,
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
    pub id: String,
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
    pub id: String,
    pub content: String,
    pub file_href: Option<String>,
    pub is_graded: bool,
    pub lecturer_notes: String,
    pub score: f32, // Pakai f32 untuk nilai yang mungkin desimal
    pub date_submitted: String, // Simpan sebagai string dulu
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub course_id: String,
    pub topic_id: String,
    pub token: String, // Untuk submit/delete
    pub title: String,
    pub description: String,
    pub file: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub status: TaskStatus,
    pub answer: Option<Answer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicDetail {
    pub id: String,
    pub access_time: Option<String>,
    pub is_accessible: bool,
    pub href: String,
    pub description: Option<String>,
    pub contents: Vec<Content>,
    pub tasks: Vec<Task>,
}
