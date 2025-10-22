use leptos::*;
use leptos_router::A;

use crate::components::use_organization_context;

#[component]
pub fn Organizations() -> impl IntoView {
    let org_ctx = use_organization_context();

    let (show_create_modal, set_show_create_modal) = create_signal(false);
    let (name, set_name) = create_signal(String::new());
    let (slug, set_slug) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (local_error, set_local_error) = create_signal(Option::<String>::None);

    // Auto-generate slug from name
    let handle_name_change = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        set_name.set(value.clone());
        
        // Generate slug: lowercase, replace spaces with hyphens
        let generated_slug = value
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "-")
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        
        set_slug.set(generated_slug);
    };

    let handle_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_local_error.set(None);

        let name_val = name.get();
        let slug_val = slug.get();
        let desc_val = description.get();

        if name_val.is_empty() {
            set_local_error.set(Some("Organization name is required".to_string()));
            return;
        }

        if slug_val.is_empty() {
            set_local_error.set(Some("Organization slug is required".to_string()));
            return;
        }

        // Validate slug
        if !slug_val.chars().all(|c| c.is_alphanumeric() || c == '-') {
            set_local_error.set(Some("Slug can only contain letters, numbers, and hyphens".to_string()));
            return;
        }

        let desc_opt = if desc_val.is_empty() {
            None
        } else {
            Some(desc_val)
        };

        spawn_local(async move {
            match org_ctx.create_organization(name_val, slug_val, desc_opt).await {
                Ok(_) => {
                    // Reset form
                    set_name.set(String::new());
                    set_slug.set(String::new());
                    set_description.set(String::new());
                    set_show_create_modal.set(false);
                    set_local_error.set(None);
                }
                Err(e) => {
                    set_local_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="min-h-screen bg-gray-50 py-8">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                // Header
                <div class="flex justify-between items-center mb-8">
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">"Organizations"</h1>
                        <p class="mt-2 text-sm text-gray-600">"Manage your organizations and teams"</p>
                    </div>
                    <button
                        on:click=move |_| set_show_create_modal.set(true)
                        class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 transition-colors"
                    >
                        "Create Organization"
                    </button>
                </div>

                // Error display
                <Show when=move || org_ctx.error.get().is_some()>
                    <div class="mb-6 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
                        {move || org_ctx.error.get().unwrap_or_default()}
                    </div>
                </Show>

                // Loading state
                <Show
                    when=move || org_ctx.is_loading.get()
                    fallback=|| view! { <div></div> }
                >
                    <div class="flex justify-center items-center py-12">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
                    </div>
                </Show>

                // Organizations grid
                <Show when=move || !org_ctx.is_loading.get()>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <For
                            each=move || org_ctx.organizations.get()
                            key=|org| org.id
                            children=move |org| {
                                let org_id = org.id;
                                let org_name = org.name.clone();
                                let org_slug = org.slug.clone();
                                let org_role = org.role.clone();
                                let has_desc = org.description.is_some();
                                let desc_text = org.description.clone().unwrap_or_default();
                                view! {
                                    <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow">
                                        <div class="flex items-start justify-between">
                                            <div class="flex-1">
                                                <h3 class="text-lg font-semibold text-gray-900">{org_name}</h3>
                                                <p class="text-sm text-gray-500 mt-1">{"@"}{org_slug}</p>
                                            </div>
                                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800">
                                                {org_role.unwrap_or_else(|| "member".to_string())}
                                            </span>
                                        </div>
                                        
                                        <Show when=move || has_desc>
                                            <p class="mt-3 text-sm text-gray-600">
                                                {desc_text.clone()}
                                            </p>
                                        </Show>

                                        <div class="mt-4 pt-4 border-t border-gray-200">
                                            <A
                                                href=format!("/organizations/{}", org_id)
                                                class="text-sm font-medium text-indigo-600 hover:text-indigo-500"
                                            >
                                                "View details →"
                                            </A>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Empty state
                    <Show when=move || org_ctx.organizations.get().is_empty()>
                        <div class="text-center py-12">
                            <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                            </svg>
                            <h3 class="mt-2 text-sm font-medium text-gray-900">"No organizations"</h3>
                            <p class="mt-1 text-sm text-gray-500">"Get started by creating a new organization."</p>
                            <div class="mt-6">
                                <button
                                    on:click=move |_| set_show_create_modal.set(true)
                                    class="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
                                >
                                    "Create Organization"
                                </button>
                            </div>
                        </div>
                    </Show>
                </Show>

                // Create Organization Modal
                <Show when=move || show_create_modal.get()>
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
                        <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
                            <div class="px-6 py-4 border-b border-gray-200">
                                <h3 class="text-lg font-medium text-gray-900">"Create New Organization"</h3>
                            </div>
                            
                            <form on:submit=handle_create class="px-6 py-4">
                                // Error display
                                <Show when=move || local_error.get().is_some()>
                                    <div class="mb-4 bg-red-50 border border-red-200 text-red-700 px-3 py-2 rounded text-sm">
                                        {move || local_error.get().unwrap_or_default()}
                                    </div>
                                </Show>

                                <div class="space-y-4">
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700">"Organization Name"</label>
                                        <input
                                            type="text"
                                            on:input=handle_name_change
                                            prop:value=move || name.get()
                                            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                                            placeholder="My Organization"
                                            required
                                        />
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-gray-700">"Slug"</label>
                                        <input
                                            type="text"
                                            on:input=move |ev| set_slug.set(event_target_value(&ev))
                                            prop:value=move || slug.get()
                                            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                                            placeholder="my-organization"
                                            required
                                        />
                                        <p class="mt-1 text-xs text-gray-500">"Used in URLs. Only letters, numbers, and hyphens."</p>
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-gray-700">"Description (Optional)"</label>
                                        <textarea
                                            on:input=move |ev| set_description.set(event_target_value(&ev))
                                            prop:value=move || description.get()
                                            rows="3"
                                            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                                            placeholder="What is this organization about?"
                                        />
                                    </div>
                                </div>

                                <div class="mt-6 flex justify-end space-x-3">
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            set_show_create_modal.set(false);
                                            set_local_error.set(None);
                                        }
                                        class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="submit"
                                        class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700"
                                    >
                                        "Create"
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
