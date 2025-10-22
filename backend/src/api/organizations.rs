use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{AuthUser, MemberRole, Organization, OrganizationMember},
    utils::AppError,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub is_active: bool,
    pub role: Option<String>, // User's role in this org
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub user_first_name: Option<String>,
    pub user_last_name: Option<String>,
    pub role: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

// Create a new organization
pub async fn create_organization(
    State(app_state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateOrganizationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

    // Validate slug (alphanumeric and hyphens only)
    if !req.slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(AppError::Validation(
            "Slug can only contain letters, numbers, and hyphens".to_string(),
        ));
    }

    // Check if slug already exists
    let exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM organizations WHERE slug = $1)")
        .bind(&req.slug)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if exists.unwrap_or(false) {
        return Err(AppError::Conflict(format!(
            "Organization with slug '{}' already exists",
            req.slug
        )));
    }

    // Create organization
    let org: Organization = sqlx::query_as(
        r#"
        INSERT INTO organizations (name, slug, description)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&req.slug)
    .bind(&req.description)
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Add creator as owner
    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(org.id)
    .bind(user_id)
    .bind(MemberRole::Owner.as_str())
    .execute(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response = OrganizationResponse {
        id: org.id,
        name: org.name,
        slug: org.slug,
        description: org.description,
        logo_url: org.logo_url,
        website: org.website,
        is_active: org.is_active,
        role: Some(MemberRole::Owner.as_str().to_string()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// Get all organizations for the current user
pub async fn get_my_organizations(
    State(app_state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

    #[derive(sqlx::FromRow)]
    struct OrgWithRole {
        id: Uuid,
        name: String,
        slug: String,
        description: Option<String>,
        logo_url: Option<String>,
        website: Option<String>,
        is_active: bool,
        role: String,
    }

    let orgs: Vec<OrgWithRole> = sqlx::query_as(
        r#"
        SELECT o.*, om.role
        FROM organizations o
        INNER JOIN organization_members om ON o.id = om.organization_id
        WHERE om.user_id = $1 AND o.is_active = true
        ORDER BY o.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response: Vec<OrganizationResponse> = orgs
        .into_iter()
        .map(|org| OrganizationResponse {
            id: org.id,
            name: org.name,
            slug: org.slug,
            description: org.description,
            logo_url: org.logo_url,
            website: org.website,
            is_active: org.is_active,
            role: Some(org.role),
        })
        .collect();

    Ok(Json(response))
}

// Get organization by ID
pub async fn get_organization(
    State(app_state): State<AppState>,
    auth: AuthUser,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

    // Check if user is member of the organization
    let member: Option<OrganizationMember> = sqlx::query_as(
        "SELECT * FROM organization_members WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org_id)
    .bind(user_id)
    .fetch_optional(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if member.is_none() {
        return Err(AppError::NotFound(
            "Organization not found or you don't have access".to_string(),
        ));
    }

    let org: Organization = sqlx::query_as("SELECT * FROM organizations WHERE id = $1 AND is_active = true")
        .bind(org_id)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

    let response = OrganizationResponse {
        id: org.id,
        name: org.name,
        slug: org.slug,
        description: org.description,
        logo_url: org.logo_url,
        website: org.website,
        is_active: org.is_active,
        role: Some(member.unwrap().role),
    };

    Ok(Json(response))
}

// Get organization members
pub async fn get_organization_members(
    State(app_state): State<AppState>,
    auth: AuthUser,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

    // Verify user is member
    let is_member: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = $1 AND user_id = $2)",
    )
    .bind(org_id)
    .bind(user_id)
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if !is_member.unwrap_or(false) {
        return Err(AppError::NotFound(
            "Organization not found or you don't have access".to_string(),
        ));
    }

    #[derive(sqlx::FromRow)]
    struct MemberWithUser {
        id: Uuid,
        user_id: Uuid,
        role: String,
        joined_at: chrono::DateTime<chrono::Utc>,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
    }

    let members: Vec<MemberWithUser> = sqlx::query_as(
        r#"
        SELECT 
            om.id, om.user_id, om.role, om.joined_at,
            u.email, u.first_name, u.last_name
        FROM organization_members om
        INNER JOIN users u ON om.user_id = u.id
        WHERE om.organization_id = $1
        ORDER BY om.joined_at ASC
        "#,
    )
    .bind(org_id)
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response: Vec<OrganizationMemberResponse> = members
        .into_iter()
        .map(|m| OrganizationMemberResponse {
            id: m.id,
            user_id: m.user_id,
            user_email: m.email,
            user_first_name: m.first_name,
            user_last_name: m.last_name,
            role: m.role,
            joined_at: m.joined_at,
        })
        .collect();

    Ok(Json(response))
}
