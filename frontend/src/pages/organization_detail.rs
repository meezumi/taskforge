use leptos::*;
use leptos_router::{use_params_map, A};
use uuid::Uuid;

use crate::services::organizations::{self, Organization, OrganizationMember};

#[component]
pub fn OrganizationDetail() -> impl IntoView {
    let params = use_params_map();
    let org_id = move || {
        params.with(|p| {
            p.get("org_id")
                .and_then(|id| Uuid::parse_str(id).ok())
        })
    };

    let (organization, set_organization) = create_signal(Option::<Organization>::None);
    let (members, set_members) = create_signal(Vec::<OrganizationMember>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error, set_error) = create_signal(Option::<String>::None);

    // Load organization and members
    create_effect(move |_| {
        if let Some(id) = org_id() {
            spawn_local(async move {
                set_is_loading.set(true);
                set_error.set(None);

                // Load organization
                match organizations::get_organization(id).await {
                    Ok(org) => set_organization.set(Some(org)),
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load organization: {}", e)));
                        set_is_loading.set(false);
                        return;
                    }
                }

                // Load members
                match organizations::get_organization_members(id).await {
                    Ok(m) => set_members.set(m),
                    Err(e) => set_error.set(Some(format!("Failed to load members: {}", e))),
                }

                set_is_loading.set(false);
            });
        }
    });

    view! {
        <div class="min-h-screen bg-gray-50 py-8">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                // Back button
                <div class="mb-6">
                    <A href="/organizations" class="text-sm text-indigo-600 hover:text-indigo-500">
                        "← Back to Organizations"
                    </A>
                </div>

                // Error display
                <Show when=move || error.get().is_some()>
                    <div class="mb-6 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
                        {move || error.get().unwrap_or_default()}
                    </div>
                </Show>

                // Loading state
                <Show
                    when=move || is_loading.get()
                    fallback=|| view! { <div></div> }
                >
                    <div class="flex justify-center items-center py-12">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
                    </div>
                </Show>

                // Organization details
                <Show when=move || !is_loading.get() && organization.get().is_some()>
                    {move || {
                        let org = organization.get().unwrap();
                        let org_name = org.name.clone();
                        let org_slug = org.slug.clone();
                        let org_role = org.role.clone();
                        let has_desc = org.description.is_some();
                        let desc_text = org.description.clone().unwrap_or_default();
                        let has_website = org.website.is_some();
                        let website_url = org.website.clone().unwrap_or_default();
                        view! {
                            <div>
                                // Organization header
                                <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
                                    <div class="flex items-start justify-between">
                                        <div>
                                            <h1 class="text-3xl font-bold text-gray-900">{org_name.clone()}</h1>
                                            <p class="text-sm text-gray-500 mt-1">{"@"}{org_slug.clone()}</p>
                                        </div>
                                        <div class="flex items-center space-x-3">
                                            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-indigo-100 text-indigo-800">
                                                {org_role.unwrap_or_else(|| "member".to_string())}
                                            </span>
                                            <A
                                                href=format!("/organizations/{}/projects", org.id)
                                                class="px-4 py-2 bg-indigo-600 text-white text-sm font-medium rounded-lg hover:bg-indigo-700 transition-colors"
                                            >
                                                "View Projects"
                                            </A>
                                        </div>
                                    </div>

                                    <Show when=move || has_desc>
                                        <p class="mt-4 text-gray-600">
                                            {desc_text.clone()}
                                        </p>
                                    </Show>

                                    <div class="mt-6 grid grid-cols-1 md:grid-cols-3 gap-4">
                                        <Show when=move || has_website>
                                            <div>
                                                <p class="text-sm font-medium text-gray-500">"Website"</p>
                                                <a
                                                    href=website_url.clone()
                                                    target="_blank"
                                                    class="text-sm text-indigo-600 hover:text-indigo-500"
                                                >
                                                    {website_url.clone()}
                                                </a>
                                            </div>
                                        </Show>
                                    </div>
                                </div>

                                // Members section
                                <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                                    <div class="flex items-center justify-between mb-4">
                                        <h2 class="text-xl font-semibold text-gray-900">"Members"</h2>
                                        <span class="text-sm text-gray-500">
                                            {move || members.get().len()} " members"
                                        </span>
                                    </div>

                                    <div class="space-y-3">
                                        <For
                                            each=move || members.get()
                                            key=|member| member.id
                                            children=move |member| {
                                                let display_name = if member.user_first_name.is_some() || member.user_last_name.is_some() {
                                                    format!(
                                                        "{} {}",
                                                        member.user_first_name.clone().unwrap_or_default(),
                                                        member.user_last_name.clone().unwrap_or_default()
                                                    ).trim().to_string()
                                                } else {
                                                    member.user_email.clone()
                                                };

                                                view! {
                                                    <div class="flex items-center justify-between py-3 border-b border-gray-200 last:border-0">
                                                        <div class="flex items-center space-x-3">
                                                            <div class="flex-shrink-0 h-10 w-10 rounded-full bg-indigo-100 flex items-center justify-center">
                                                                <span class="text-indigo-600 font-medium text-sm">
                                                                    {display_name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                                                </span>
                                                            </div>
                                                            <div>
                                                                <p class="text-sm font-medium text-gray-900">{display_name}</p>
                                                                <p class="text-xs text-gray-500">{member.user_email.clone()}</p>
                                                            </div>
                                                        </div>
                                                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                                                            {member.role.clone()}
                                                        </span>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>

                                    // Empty state
                                    <Show when=move || members.get().is_empty()>
                                        <div class="text-center py-8">
                                            <p class="text-sm text-gray-500">"No members yet"</p>
                                        </div>
                                    </Show>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}
