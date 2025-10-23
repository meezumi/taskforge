use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use uuid::Uuid;

use crate::{
    AppState,
    models::{AuthUser, CreateProjectRequest, Project, ProjectResponse, UpdateProjectRequest},
    utils::AppError,
};

pub async fn create_project(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(org_id): Path<Uuid>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ProjectResponse>), AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user is a member of the organization
    let membership = sqlx::query!(
        r#"SELECT role FROM organization_members WHERE organization_id = $1 AND user_id = $2"#,
        org_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check organization membership".to_string())
    })?;

    if membership.is_none() {
        return Err(AppError::Authorization(
            "You are not a member of this organization".to_string(),
        ));
    }

    // Check if slug is unique within organization
    let existing = sqlx::query!(
        "SELECT id FROM projects WHERE organization_id = $1 AND slug = $2",
        org_id,
        payload.slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check project slug".to_string())
    })?;

    if existing.is_some() {
        return Err(AppError::Conflict(
            "A project with this slug already exists in this organization".to_string(),
        ));
    }

    let status = payload.status.unwrap_or_else(|| "planning".to_string());
    let color = payload.color.unwrap_or_else(|| "#3B82F6".to_string());

    let project = sqlx::query_as!(
        Project,
        r#"
        INSERT INTO projects (organization_id, name, slug, description, status, color, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, organization_id, name, slug, description, status, color, created_by, created_at, updated_at
        "#,
        org_id,
        payload.name,
        payload.slug,
        payload.description,
        status,
        color,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to create project".to_string())
    })?;

    tracing::info!("Project created: {} in org {}", project.name, org_id);

    Ok((StatusCode::CREATED, Json(project.into())))
}

pub async fn get_organization_projects(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(org_id): Path<Uuid>,
) -> Result<Json<Vec<ProjectResponse>>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user is a member of the organization
    let membership = sqlx::query!(
        "SELECT id FROM organization_members WHERE organization_id = $1 AND user_id = $2",
        org_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check organization membership".to_string())
    })?;

    if membership.is_none() {
        return Err(AppError::Authorization(
            "You are not a member of this organization".to_string(),
        ));
    }

    let projects = sqlx::query_as!(
        Project,
        r#"
        SELECT id, organization_id, name, slug, description, status, color, created_by, created_at, updated_at
        FROM projects
        WHERE organization_id = $1
        ORDER BY created_at DESC
        "#,
        org_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to fetch projects".to_string())
    })?;

    let responses: Vec<ProjectResponse> = projects.into_iter().map(|p| p.into()).collect();

    Ok(Json(responses))
}

pub async fn get_project(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    let project = sqlx::query_as!(
        Project,
        r#"
        SELECT p.id, p.organization_id, p.name, p.slug, p.description, p.status, p.color, p.created_by, p.created_at, p.updated_at
        FROM projects p
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.id = $1 AND om.user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to fetch project".to_string())
    })?;

    match project {
        Some(p) => Ok(Json(p.into())),
        None => Err(AppError::NotFound("Project not found".to_string())),
    }
}

pub async fn update_project(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user has access to the project's organization
    let access = sqlx::query!(
        r#"
        SELECT om.role
        FROM projects p
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.id = $1 AND om.user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check project access".to_string())
    })?;

    if access.is_none() {
        return Err(AppError::Authorization("You don't have access to this project".to_string()));
    }

    // Build dynamic update query
    let project = sqlx::query_as!(
        Project,
        r#"
        UPDATE projects
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            status = COALESCE($4, status),
            color = COALESCE($5, color),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, organization_id, name, slug, description, status, color, created_by, created_at, updated_at
        "#,
        project_id,
        payload.name,
        payload.description,
        payload.status,
        payload.color
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to update project".to_string())
    })?;

    tracing::info!("Project updated: {}", project.id);

    Ok(Json(project.into()))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user is owner or admin of the organization
    let access = sqlx::query!(
        r#"
        SELECT om.role
        FROM projects p
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.id = $1 AND om.user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check project access".to_string())
    })?;

    match access {
        Some(record) if record.role == "owner" || record.role == "admin" => {
            sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    tracing::error!("Database error: {}", e);
                    AppError::DatabaseError("Failed to delete project".to_string())
                })?;

            tracing::info!("Project deleted: {}", project_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Some(_) => Err(AppError::Authorization(
            "Only organization owners and admins can delete projects".to_string(),
        )),
        None => Err(AppError::NotFound("Project not found".to_string())),
    }
}
