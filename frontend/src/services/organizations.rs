use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::api::{get, post, ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub is_active: bool,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub user_first_name: Option<String>,
    pub user_last_name: Option<String>,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

/// Get all organizations for the current user
pub async fn get_my_organizations() -> Result<Vec<Organization>, ApiError> {
    get("/api/organizations").await
}

/// Create a new organization
pub async fn create_organization(
    name: String,
    slug: String,
    description: Option<String>,
) -> Result<Organization, ApiError> {
    let request = CreateOrganizationRequest {
        name,
        slug,
        description,
    };
    post("/api/organizations", &request).await
}

/// Get a single organization by ID
pub async fn get_organization(org_id: Uuid) -> Result<Organization, ApiError> {
    get(&format!("/api/organizations/{}", org_id)).await
}

/// Get organization members
pub async fn get_organization_members(org_id: Uuid) -> Result<Vec<OrganizationMember>, ApiError> {
    get(&format!("/api/organizations/{}/members", org_id)).await
}
