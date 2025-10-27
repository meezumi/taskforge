// API routes module
pub mod auth;
pub mod organizations;
pub mod projects;
pub mod tasks;

pub use auth::{login, me, register};
pub use organizations::{
    create_organization, get_my_organizations, get_organization, get_organization_members,
};
pub use projects::{
    create_project, delete_project, get_organization_projects, get_project, update_project,
};
pub use tasks::{
    create_comment, create_task, delete_task, get_project_tasks, get_task, get_task_comments,
    update_task,
};

