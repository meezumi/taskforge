use leptos::*;
use leptos_router::*;

use crate::services::projects::{self, CreateProjectRequest, Project};

#[component]
pub fn Projects() -> impl IntoView {
    let params = use_params_map();
    let org_id = move || {
        params.with(|p| p.get("org_id").cloned().unwrap_or_default())
    };

    let (projects, set_projects) = create_signal::<Vec<Project>>(vec![]);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (show_create_modal, set_show_create_modal) = create_signal(false);

    // Form state
    let (name, set_name) = create_signal(String::new());
    let (slug, set_slug) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (status, set_status) = create_signal(String::from("planning"));
    let (color, set_color) = create_signal(String::from("#3B82F6"));
    let (create_error, set_create_error) = create_signal::<Option<String>>(None);

    // Load projects
    create_effect(move |_| {
        let org_id_val = org_id();
        if !org_id_val.is_empty() {
            spawn_local(async move {
                set_loading.set(true);
                match projects::get_organization_projects(&org_id_val).await {
                    Ok(proj_list) => {
                        set_projects.set(proj_list);
                        set_error.set(None);
                    }
                    Err(e) => {
                        log::error!("Failed to load projects: {}", e);
                        set_error.set(Some(format!("Failed to load projects: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }
    });

    // Auto-generate slug from name
    create_effect(move |_| {
        let n = name.get();
        if !n.is_empty() {
            let auto_slug = n
                .to_lowercase()
                .replace(" ", "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            set_slug.set(auto_slug);
        }
    });

    let handle_create = move |_| {
        let name_val = name.get();
        let slug_val = slug.get();
        let org_id_val = org_id();
        
        if name_val.is_empty() {
            set_create_error.set(Some("Project name is required".to_string()));
            return;
        }

        if slug_val.is_empty() {
            set_create_error.set(Some("Project slug is required".to_string()));
            return;
        }

        spawn_local(async move {
            let request = CreateProjectRequest {
                name: name_val.clone(),
                slug: slug_val.clone(),
                description: if description.get().is_empty() {
                    None
                } else {
                    Some(description.get())
                },
                status: Some(status.get()),
                color: Some(color.get()),
            };

            match projects::create_project(&org_id_val, request).await {
                Ok(new_project) => {
                    log::info!("Project created: {:?}", new_project);
                    set_projects.update(|p| p.push(new_project));
                    set_show_create_modal.set(false);
                    set_name.set(String::new());
                    set_slug.set(String::new());
                    set_description.set(String::new());
                    set_status.set(String::from("planning"));
                    set_color.set(String::from("#3B82F6"));
                    set_create_error.set(None);
                }
                Err(e) => {
                    log::error!("Failed to create project: {}", e);
                    set_create_error.set(Some(format!("Failed to create project: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div class="flex justify-between items-center mb-8">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">"Projects"</h1>
                    <p class="mt-2 text-gray-600">"Organize your work into projects"</p>
                </div>
                <button
                    on:click=move |_| set_show_create_modal.set(true)
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                    "Create Project"
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
                } else if projects.get().is_empty() {
                    view! {
                        <div class="text-center py-12">
                            <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            <h3 class="mt-2 text-sm font-medium text-gray-900">"No projects"</h3>
                            <p class="mt-1 text-sm text-gray-500">"Get started by creating a new project."</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            <For
                                each=move || projects.get()
                                key=|project| project.id
                                children=move |project: Project| {
                                    let project_id = project.id.to_string();
                                    let project_org_id = org_id();
                                    let color = project.color.clone().unwrap_or_else(|| "#3B82F6".to_string());
                                    
                                    view! {
                                        <a
                                            href=format!("/organizations/{}/projects/{}", project_org_id, project_id)
                                            class="block bg-white rounded-lg border-2 hover:border-gray-300 transition-colors"
                                            style=format!("border-color: {}", color)
                                        >
                                            <div class="p-6">
                                                <div class="flex items-center mb-2">
                                                    <div
                                                        class="w-3 h-3 rounded-full mr-2"
                                                        style=format!("background-color: {}", color)
                                                    ></div>
                                                    <h3 class="text-xl font-semibold text-gray-900">{project.name.clone()}</h3>
                                                </div>
                                                <p class="text-sm text-gray-500 mb-4">
                                                    {project.slug.clone()}
                                                </p>
                                                {project.description.clone().map(|desc| {
                                                    view! {
                                                        <p class="text-gray-600 line-clamp-2">{desc}</p>
                                                    }
                                                })}
                                                <div class="mt-4 pt-4 border-t border-gray-100">
                                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                                        {project.status.clone()}
                                                    </span>
                                                </div>
                                            </div>
                                        </a>
                                    }
                                }
                            />
                        </div>
                    }.into_view()
                }
            }}

            // Create Project Modal
            {move || if show_create_modal.get() {
                view! {
                    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
                        <div class="bg-white rounded-lg max-w-md w-full p-6">
                            <h2 class="text-2xl font-bold mb-4">"Create New Project"</h2>
                            
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
                                        "Project Name"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="My Awesome Project"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        prop:value=move || name.get()
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                    />
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Project Slug"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="my-awesome-project"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        prop:value=move || slug.get()
                                        on:input=move |ev| set_slug.set(event_target_value(&ev))
                                    />
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Description"
                                    </label>
                                    <textarea
                                        placeholder="What's this project about?"
                                        rows="3"
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        prop:value=move || description.get()
                                        on:input=move |ev| set_description.set(event_target_value(&ev))
                                    ></textarea>
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Status"
                                    </label>
                                    <select
                                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        on:change=move |ev| set_status.set(event_target_value(&ev))
                                    >
                                        <option value="planning">"Planning"</option>
                                        <option value="active">"Active"</option>
                                        <option value="on-hold">"On Hold"</option>
                                        <option value="completed">"Completed"</option>
                                        <option value="archived">"Archived"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">
                                        "Color"
                                    </label>
                                    <input
                                        type="color"
                                        class="w-full h-10 border border-gray-300 rounded-lg cursor-pointer"
                                        prop:value=move || color.get()
                                        on:input=move |ev| set_color.set(event_target_value(&ev))
                                    />
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
                                    "Create Project"
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
