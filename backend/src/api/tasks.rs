use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use uuid::Uuid;

use crate::{
    AppState,
    models::{AuthUser, CreateTaskRequest, Task, TaskResponse, UpdateTaskRequest, CreateCommentRequest, TaskComment, CommentResponse},
    utils::AppError,
};

pub async fn create_task(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>), AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user has access to the project's organization
    let access = sqlx::query!(
        r#"
        SELECT om.id
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
        return Err(AppError::Authorization(
            "You don't have access to this project".to_string(),
        ));
    }

    let status = payload.status.unwrap_or_else(|| "todo".to_string());
    let priority = payload.priority.unwrap_or_else(|| "medium".to_string());

    // Get the next position for this status
    let max_position = sqlx::query!(
        "SELECT COALESCE(MAX(position), -1) as max_pos FROM tasks WHERE project_id = $1 AND status = $2",
        project_id,
        status
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to get max position".to_string())
    })?;

    let position = max_position.max_pos.unwrap_or(-1) + 1;

    let task = sqlx::query_as!(
        Task,
        r#"
        INSERT INTO tasks (project_id, title, description, status, priority, assigned_to, created_by, due_date, position)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, project_id, title, description, status, priority, assigned_to, created_by, due_date, completed_at, position, created_at, updated_at
        "#,
        project_id,
        payload.title,
        payload.description,
        status,
        priority,
        payload.assigned_to,
        user_id,
        payload.due_date,
        position
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to create task".to_string())
    })?;

    tracing::info!("Task created: {} in project {}", task.title, project_id);

    Ok((StatusCode::CREATED, Json(task.into())))
}

pub async fn get_project_tasks(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check if user has access to the project
    let access = sqlx::query!(
        r#"
        SELECT om.id
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
        return Err(AppError::Authorization(
            "You don't have access to this project".to_string(),
        ));
    }

    let tasks = sqlx::query_as!(
        Task,
        r#"
        SELECT id, project_id, title, description, status, priority, assigned_to, created_by, due_date, completed_at, position, created_at, updated_at
        FROM tasks
        WHERE project_id = $1
        ORDER BY position ASC, created_at ASC
        "#,
        project_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to fetch tasks".to_string())
    })?;

    let responses: Vec<TaskResponse> = tasks.into_iter().map(|t| t.into()).collect();

    Ok(Json(responses))
}

pub async fn get_task(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TaskResponse>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT t.id, t.project_id, t.title, t.description, t.status, t.priority, t.assigned_to, t.created_by, t.due_date, t.completed_at, t.position, t.created_at, t.updated_at
        FROM tasks t
        INNER JOIN projects p ON t.project_id = p.id
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE t.id = $1 AND om.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to fetch task".to_string())
    })?;

    match task {
        Some(t) => Ok(Json(t.into())),
        None => Err(AppError::NotFound("Task not found".to_string())),
    }
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check access
    let access = sqlx::query!(
        r#"
        SELECT om.id
        FROM tasks t
        INNER JOIN projects p ON t.project_id = p.id
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE t.id = $1 AND om.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check task access".to_string())
    })?;

    if access.is_none() {
        return Err(AppError::Authorization("You don't have access to this task".to_string()));
    }

    // Check if task is being marked as completed
    let completed_at = if let Some(ref status) = payload.status {
        if status == "done" {
            Some(chrono::Utc::now())
        } else {
            None
        }
    } else {
        None
    };

    let task = sqlx::query_as!(
        Task,
        r#"
        UPDATE tasks
        SET 
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            status = COALESCE($4, status),
            priority = COALESCE($5, priority),
            assigned_to = COALESCE($6, assigned_to),
            due_date = COALESCE($7, due_date),
            position = COALESCE($8, position),
            completed_at = COALESCE($9, completed_at),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, project_id, title, description, status, priority, assigned_to, created_by, due_date, completed_at, position, created_at, updated_at
        "#,
        task_id,
        payload.title,
        payload.description,
        payload.status,
        payload.priority,
        payload.assigned_to,
        payload.due_date,
        payload.position,
        completed_at
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to update task".to_string())
    })?;

    tracing::info!("Task updated: {}", task.id);

    Ok(Json(task.into()))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check access
    let access = sqlx::query!(
        r#"
        SELECT om.role
        FROM tasks t
        INNER JOIN projects p ON t.project_id = p.id
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE t.id = $1 AND om.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check task access".to_string())
    })?;

    match access {
        Some(_) => {
            sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    tracing::error!("Database error: {}", e);
                    AppError::DatabaseError("Failed to delete task".to_string())
                })?;

            tracing::info!("Task deleted: {}", task_id);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(AppError::NotFound("Task not found".to_string())),
    }
}

pub async fn create_comment(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check access
    let access = sqlx::query!(
        r#"
        SELECT om.id
        FROM tasks t
        INNER JOIN projects p ON t.project_id = p.id
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE t.id = $1 AND om.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check task access".to_string())
    })?;

    if access.is_none() {
        return Err(AppError::Authorization("You don't have access to this task".to_string()));
    }

    let comment = sqlx::query_as!(
        TaskComment,
        r#"
        INSERT INTO task_comments (task_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING id, task_id, user_id, content, created_at, updated_at
        "#,
        task_id,
        user_id,
        payload.content
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to create comment".to_string())
    })?;

    tracing::info!("Comment created on task: {}", task_id);

    Ok((StatusCode::CREATED, Json(comment.into())))
}

pub async fn get_task_comments(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let user_id = Uuid::parse_str(&auth_user.user_id)
        .map_err(|_| AppError::Authentication("Invalid user ID".to_string()))?;

    // Check access
    let access = sqlx::query!(
        r#"
        SELECT om.id
        FROM tasks t
        INNER JOIN projects p ON t.project_id = p.id
        INNER JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE t.id = $1 AND om.user_id = $2
        "#,
        task_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to check task access".to_string())
    })?;

    if access.is_none() {
        return Err(AppError::Authorization("You don't have access to this task".to_string()));
    }

    let comments = sqlx::query_as!(
        TaskComment,
        r#"
        SELECT id, task_id, user_id, content, created_at, updated_at
        FROM task_comments
        WHERE task_id = $1
        ORDER BY created_at ASC
        "#,
        task_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError("Failed to fetch comments".to_string())
    })?;

    let responses: Vec<CommentResponse> = comments.into_iter().map(|c| c.into()).collect();

    Ok(Json(responses))
}
