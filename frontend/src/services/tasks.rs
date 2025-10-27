use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub assigned_to: Option<Uuid>,
    pub created_by: Uuid,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

pub async fn get_project_tasks(project_id: &str) -> Result<Vec<Task>, String> {
    let endpoint = format!("/api/projects/{}/tasks", project_id);
    api::get::<Vec<Task>>(&endpoint)
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_task(project_id: &str, request: CreateTaskRequest) -> Result<Task, String> {
    let endpoint = format!("/api/projects/{}/tasks", project_id);
    api::post::<CreateTaskRequest, Task>(&endpoint, &request)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_task(task_id: &str) -> Result<Task, String> {
    let endpoint = format!("/api/tasks/{}", task_id);
    api::get::<Task>(&endpoint)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_task(task_id: &str, request: UpdateTaskRequest) -> Result<Task, String> {
    let endpoint = format!("/api/tasks/{}", task_id);
    api::put::<UpdateTaskRequest, Task>(&endpoint, &request)
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_task(task_id: &str) -> Result<(), String> {
    let endpoint = format!("/api/tasks/{}", task_id);
    api::delete(&endpoint).await
}

pub async fn get_task_comments(task_id: &str) -> Result<Vec<Comment>, String> {
    let endpoint = format!("/api/tasks/{}/comments", task_id);
    api::get::<Vec<Comment>>(&endpoint)
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_comment(task_id: &str, request: CreateCommentRequest) -> Result<Comment, String> {
    let endpoint = format!("/api/tasks/{}/comments", task_id);
    api::post::<CreateCommentRequest, Comment>(&endpoint, &request)
        .await
        .map_err(|e| e.to_string())
}
