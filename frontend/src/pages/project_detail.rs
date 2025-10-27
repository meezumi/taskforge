use leptos::*;
use leptos_router::*;

use crate::services::tasks::{self, CreateTaskRequest, Task, UpdateTaskRequest};

#[component]
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || {
        params.with(|p| p.get("project_id").cloned().unwrap_or_default())
    };
    let org_id = move || {
        params.with(|p| p.get("org_id").cloned().unwrap_or_default())
    };

    let (tasks, set_tasks) = create_signal::<Vec<Task>>(vec![]);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (show_create_modal, set_show_create_modal) = create_signal(false);

    // Form state
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (priority, set_priority) = create_signal(String::from("medium"));
    let (create_error, set_create_error) = create_signal::<Option<String>>(None);

    // Load tasks
    create_effect(move |_| {
        let proj_id = project_id();
        if !proj_id.is_empty() {
            spawn_local(async move {
                set_loading.set(true);
                match tasks::get_project_tasks(&proj_id).await {
                    Ok(task_list) => {
                        set_tasks.set(task_list);
                        set_error.set(None);
                    }
                    Err(e) => {
                        log::error!("Failed to load tasks: {}", e);
                        set_error.set(Some(format!("Failed to load tasks: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }
    });

    let handle_create = move |_| {
        let title_val = title.get();
        let proj_id = project_id();
        
        if title_val.is_empty() {
            set_create_error.set(Some("Task title is required".to_string()));
            return;
        }

        spawn_local(async move {
            let request = CreateTaskRequest {
                title: title_val.clone(),
                description: if description.get().is_empty() {
                    None
                } else {
                    Some(description.get())
                },
                status: Some("todo".to_string()),
                priority: Some(priority.get()),
                assigned_to: None,
                due_date: None,
            };

            match tasks::create_task(&proj_id, request).await {
                Ok(new_task) => {
                    log::info!("Task created: {:?}", new_task);
                    set_tasks.update(|t| t.push(new_task));
                    set_show_create_modal.set(false);
                    set_title.set(String::new());
                    set_description.set(String::new());
                    set_priority.set(String::from("medium"));
                    set_create_error.set(None);
                }
                Err(e) => {
                    log::error!("Failed to create task: {}", e);
                    set_create_error.set(Some(format!("Failed to create task: {}", e)));
                }
            }
        });
    };

    let handle_status_change = move |task_id: String, new_status: String| {
        spawn_local(async move {
            let request = UpdateTaskRequest {
                title: None,
                description: None,
                status: Some(new_status),
                priority: None,
                assigned_to: None,
                due_date: None,
                position: None,
            };

            match tasks::update_task(&task_id, request).await {
                Ok(updated_task) => {
                    set_tasks.update(|tasks| {
                        if let Some(task) = tasks.iter_mut().find(|t| t.id.to_string() == task_id) {
                            *task = updated_task;
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to update task: {}", e);
                }
            }
        });
    };

    // Group tasks by status
    let todo_tasks = move || {
        tasks.get().into_iter().filter(|t| t.status == "todo").collect::<Vec<_>>()
    };
    
    let in_progress_tasks = move || {
        tasks.get().into_iter().filter(|t| t.status == "in_progress").collect::<Vec<_>>()
    };
    
    let done_tasks = move || {
        tasks.get().into_iter().filter(|t| t.status == "done").collect::<Vec<_>>()
    };

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div class="flex justify-between items-center mb-8">
                <div>
                    <A href=format!("/organizations/{}/projects", org_id()) class="text-sm text-blue-600 hover:text-blue-500 mb-2 inline-block">
                        "← Back to Projects"
                    </A>
                    <h1 class="text-3xl font-bold text-gray-900">"Task Board"</h1>
                    <p class="mt-2 text-gray-600">"Manage your tasks with a Kanban board"</p>
                </div>
                <button
                    on:click=move |_| set_show_create_modal.set(true)
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                    "New Task"
                </button>
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="flex justify-center items-center py-12">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="bg-red-50 border border-red-200 rounded-lg p-4">
                            <p class="text-red-800">{err}</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            // To Do Column
                            <div class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h2 class="font-semibold text-gray-900">"To Do"</h2>
                                    <span class="text-sm text-gray-500">{move || todo_tasks().len()}</span>
                                </div>
                                <div class="space-y-3">
                                    <For
                                        each=todo_tasks
                                        key=|task| task.id
                                        children=move |task: Task| {
                                            let task_id = task.id.to_string();
                                            let task_id_for_progress = task_id.clone();
                                            let task_id_for_done = task_id.clone();
                                            let priority_color = match task.priority.as_str() {
                                                "high" => "border-l-4 border-red-500",
                                                "medium" => "border-l-4 border-yellow-500",
                                                "low" => "border-l-4 border-green-500",
                                                _ => "border-l-4 border-gray-300"
                                            };
                                            
                                            view! {
                                                <div class=format!("bg-white rounded-lg p-4 shadow-sm {}", priority_color)>
                                                    <h3 class="font-medium text-gray-900 mb-2">{task.title.clone()}</h3>
                                                    {task.description.clone().map(|desc| {
                                                        view! {
                                                            <p class="text-sm text-gray-600 mb-3">{desc}</p>
                                                        }
                                                    })}
                                                    <div class="flex items-center justify-between">
                                                        <span class="text-xs px-2 py-1 rounded-full bg-blue-100 text-blue-800">
                                                            {task.priority.clone()}
                                                        </span>
                                                        <div class="flex space-x-1">
                                                            <button
                                                                on:click=move |_| handle_status_change(task_id_for_progress.clone(), "in_progress".to_string())
                                                                class="text-xs text-blue-600 hover:text-blue-800"
                                                            >
                                                                "→"
                                                            </button>
                                                            <button
                                                                on:click=move |_| handle_status_change(task_id_for_done.clone(), "done".to_string())
                                                                class="text-xs text-green-600 hover:text-green-800"
                                                            >
                                                                "✓"
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            // In Progress Column
                            <div class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h2 class="font-semibold text-gray-900">"In Progress"</h2>
                                    <span class="text-sm text-gray-500">{move || in_progress_tasks().len()}</span>
                                </div>
                                <div class="space-y-3">
                                    <For
                                        each=in_progress_tasks
                                        key=|task| task.id
                                        children=move |task: Task| {
                                            let task_id = task.id.to_string();
                                            let task_id_for_todo = task_id.clone();
                                            let task_id_for_done = task_id.clone();
                                            let priority_color = match task.priority.as_str() {
                                                "high" => "border-l-4 border-red-500",
                                                "medium" => "border-l-4 border-yellow-500",
                                                "low" => "border-l-4 border-green-500",
                                                _ => "border-l-4 border-gray-300"
                                            };
                                            
                                            view! {
                                                <div class=format!("bg-white rounded-lg p-4 shadow-sm {}", priority_color)>
                                                    <h3 class="font-medium text-gray-900 mb-2">{task.title.clone()}</h3>
                                                    {task.description.clone().map(|desc| {
                                                        view! {
                                                            <p class="text-sm text-gray-600 mb-3">{desc}</p>
                                                        }
                                                    })}
                                                    <div class="flex items-center justify-between">
                                                        <span class="text-xs px-2 py-1 rounded-full bg-yellow-100 text-yellow-800">
                                                            {task.priority.clone()}
                                                        </span>
                                                        <div class="flex space-x-1">
                                                            <button
                                                                on:click=move |_| handle_status_change(task_id_for_todo.clone(), "todo".to_string())
                                                                class="text-xs text-gray-600 hover:text-gray-800"
                                                            >
                                                                "←"
                                                            </button>
                                                            <button
                                                                on:click=move |_| handle_status_change(task_id_for_done.clone(), "done".to_string())
                                                                class="text-xs text-green-600 hover:text-green-800"
                                                            >
                                                                "✓"
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            // Done Column
                            <div class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h2 class="font-semibold text-gray-900">"Done"</h2>
                                    <span class="text-sm text-gray-500">{move || done_tasks().len()}</span>
                                </div>
                                <div class="space-y-3">
                                    <For
                                        each=done_tasks
                                        key=|task| task.id
                                        children=move |task: Task| {
                                            let task_id = task.id.to_string();
                                            let task_id_for_progress = task_id.clone();
                                            let priority_color = match task.priority.as_str() {
                                                "high" => "border-l-4 border-red-500",
                                                "medium" => "border-l-4 border-yellow-500",
                                                "low" => "border-l-4 border-green-500",
                                                _ => "border-l-4 border-gray-300"
                                            };
                                            
                                            view! {
                                                <div class=format!("bg-white rounded-lg p-4 shadow-sm opacity-75 {}", priority_color)>
                                                    <h3 class="font-medium text-gray-900 mb-2 line-through">{task.title.clone()}</h3>
                                                    {task.description.clone().map(|desc| {
                                                        view! {
                                                            <p class="text-sm text-gray-600 mb-3">{desc}</p>
                                                        }
                                                    })}
                                                    <div class="flex items-center justify-between">
                                                        <span class="text-xs px-2 py-1 rounded-full bg-green-100 text-green-800">
                                                            {task.priority.clone()}
                                                        </span>
                                                        <button
                                                            on:click=move |_| handle_status_change(task_id_for_progress.clone(), "in_progress".to_string())
                                                            class="text-xs text-gray-600 hover:text-gray-800"
                                                        >
                                                            "←"
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }
            }}

            // Create Task Modal
            {move || if show_create_modal.get() {
                view! {
                    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
                        <div class="bg-white rounded-lg max-w-md w-full p-6">
                            <h2 class="text-2xl font-bold mb-4">"Create New Task"</h2>
                            
                            {move || create_error.get().map(|err| {
                                view! {
                                    <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
                                        <p class="text-red-800 text-sm">{err}</p>
                                    </div>
                                }
                            })}

                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Title"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="Task title"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        prop:value=move || title.get()
                                        on:input=move |ev| set_title.set(event_target_value(&ev))
                                    />
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Description"
                                    </label>
                                    <textarea
                                        placeholder="What needs to be done?"
                                        rows="3"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        prop:value=move || description.get()
                                        on:input=move |ev| set_description.set(event_target_value(&ev))
                                    ></textarea>
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Priority"
                                    </label>
                                    <select
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        on:change=move |ev| set_priority.set(event_target_value(&ev))
                                    >
                                        <option value="low">"Low"</option>
                                        <option value="medium" selected>"Medium"</option>
                                        <option value="high">"High"</option>
                                    </select>
                                </div>
                            </div>

                            <div class="flex justify-end space-x-3 mt-6">
                                <button
                                    on:click=move |_| {
                                        set_show_create_modal.set(false);
                                        set_create_error.set(None);
                                    }
                                    class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
                                >
                                    "Cancel"
                                </button>
                                <button
                                    on:click=handle_create
                                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                                >
                                    "Create Task"
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}
        </div>
    }
}
