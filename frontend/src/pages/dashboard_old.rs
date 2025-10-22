use leptos::*;
use leptos_router::use_navigate;
use crate::components::use_auth_context;

#[component]
pub fn Dashboard() -> impl IntoView {
    let auth = use_auth_context();
    let navigate = use_navigate();

    // Load user on mount
    create_effect(move |_| {
        if auth.user.get().is_none() {
            spawn_local(async move {
                auth.load_user().await;
            });
        }
    });

    let on_logout = move |_| {
        auth.logout();
        navigate("/login", Default::default());
    };

    let on_login_nav = move |_| {
        navigate("/login", Default::default());
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            // Navigation bar
            <nav class="bg-white shadow-sm">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16">
                        <div class="flex items-center">
                            <h1 class="text-2xl font-bold text-indigo-600">"TaskForge"</h1>
                        </div>
                        <div class="flex items-center space-x-4">
                            <Show
                                when=move || auth.user.get().is_some()
                                fallback=|| view! { <div></div> }
                            >
                                {move || {
                                    let user = auth.user.get().unwrap();
                                    view! {
                                        <div class="flex items-center space-x-4">
                                            <span class="text-sm text-gray-700">
                                                {user.full_name()}
                                            </span>
                                            <button
                                                on:click=on_logout
                                                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                            >
                                                "Logout"
                                            </button>
                                        </div>
                                    }
                                }}
                            </Show>
                        </div>
                    </div>
                </div>
            </nav>

            // Main content
            <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <Show
                    when=move || auth.is_loading.get()
                    fallback=move || view! {
                        <Show
                            when=move || auth.user.get().is_some()
                            fallback=move || view! {
                                <div class="flex justify-center items-center h-64">
                                    <div class="text-center">
                                        <p class="text-gray-500 mb-4">"Please log in to continue"</p>
                                        <button
                                            on:click=on_login_nav
                                            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
                                        >
                                            "Go to Login"
                                        </button>
                                    </div>
                                </div>
                            }
                        >
                            {move || {
                                let user = auth.user.get().unwrap();
                                view! {
                                    <div class="px-4 py-6 sm:px-0">
                                        <div class="bg-white shadow rounded-lg p-6">
                                            <h2 class="text-2xl font-bold text-gray-900 mb-4">
                                                "Welcome back, " {user.full_name()} "!"
                                            </h2>
                                            
                                            <div class="mt-6 border-t border-gray-200 pt-6">
                                                <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                                                    <div class="sm:col-span-1">
                                                        <dt class="text-sm font-medium text-gray-500">"Email"</dt>
                                                        <dd class="mt-1 text-sm text-gray-900">{user.email.clone()}</dd>
                                                    </div>
                                                    <div class="sm:col-span-1">
                                                        <dt class="text-sm font-medium text-gray-500">"Account Status"</dt>
                                                        <dd class="mt-1">
                                                            <span class={if user.is_active {
                                                                "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800"
                                                            } else {
                                                                "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800"
                                                            }}>
                                                                {if user.is_active { "Active" } else { "Inactive" }}
                                                            </span>
                                                        </dd>
                                                    </div>
                                                    <div class="sm:col-span-1">
                                                        <dt class="text-sm font-medium text-gray-500">"Email Verified"</dt>
                                                        <dd class="mt-1">
                                                            <span class={if user.is_email_verified {
                                                                "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800"
                                                            } else {
                                                                "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800"
                                                            }}>
                                                                {if user.is_email_verified { "Verified" } else { "Not Verified" }}
                                                            </span>
                                                        </dd>
                                                    </div>
                                                    <div class="sm:col-span-1">
                                                        <dt class="text-sm font-medium text-gray-500">"User ID"</dt>
                                                        <dd class="mt-1 text-sm text-gray-900 font-mono">{user.id.clone()}</dd>
                                                    </div>
                                                </dl>
                                            </div>
                                        </div>

                                        <div class="mt-6 bg-white shadow rounded-lg p-6">
                                            <h3 class="text-lg font-medium text-gray-900 mb-4">"Quick Actions"</h3>
                                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                                                <button class="inline-flex items-center justify-center px-4 py-3 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                                    "Create Project"
                                                </button>
                                                <button class="inline-flex items-center justify-center px-4 py-3 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                                    "View Tasks"
                                                </button>
                                                <button class="inline-flex items-center justify-center px-4 py-3 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                                    "Team Settings"
                                                </button>
                                            </div>
                                        </div>

                                        <div class="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-6">
                                            <h3 class="text-lg font-medium text-blue-900 mb-2">"🚀 Coming Soon"</h3>
                                            <p class="text-sm text-blue-700">
                                                "We're working on exciting features including project management, task tracking, team collaboration, and real-time updates. Stay tuned!"
                                            </p>
                                        </div>
                                    </div>
                                }
                            }}
                        </Show>
                    }
                >
                    <div class="flex justify-center items-center h-64">
                        <div class="text-gray-500">"Loading..."</div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
