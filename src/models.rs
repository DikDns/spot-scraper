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
