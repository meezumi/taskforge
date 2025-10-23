use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub color: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub color: Option<String>,
}

pub async fn get_organization_projects(org_id: &str) -> Result<Vec<Project>, String> {
    let endpoint = format!("/api/organizations/{}/projects", org_id);
    api::get::<Vec<Project>>(&endpoint)
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_project(org_id: &str, request: CreateProjectRequest) -> Result<Project, String> {
    let endpoint = format!("/api/organizations/{}/projects", org_id);
    api::post::<CreateProjectRequest, Project>(&endpoint, &request)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_project(project_id: &str) -> Result<Project, String> {
    let endpoint = format!("/api/projects/{}", project_id);
    api::get::<Project>(&endpoint)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_project(project_id: &str, request: UpdateProjectRequest) -> Result<Project, String> {
    let endpoint = format!("/api/projects/{}", project_id);
    api::put::<UpdateProjectRequest, Project>(&endpoint, &request)
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_project(project_id: &str) -> Result<(), String> {
    let endpoint = format!("/api/projects/{}", project_id);
    api::delete(&endpoint).await
}
