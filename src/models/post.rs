// src/models/post.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePost {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePost {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
}