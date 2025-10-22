use leptos::*;
use crate::components::use_auth_context;

#[component]
pub fn Login() -> impl IntoView {
    let auth = use_auth_context();

    // Form mode: "login" or "register"
    let (mode, set_mode) = create_signal("login");

    // Form fields
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (first_name, set_first_name) = create_signal(String::new());
    let (last_name, set_last_name) = create_signal(String::new());
    let (local_error, set_local_error) = create_signal(Option::<String>::None);

    // Handle login
    let handle_login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_local_error.set(None);

        let email_val = email.get();
        let password_val = password.get();

        if email_val.is_empty() || password_val.is_empty() {
            set_local_error.set(Some("Email and password are required".to_string()));
            return;
        }

        spawn_local(async move {
            match auth.login(email_val, password_val).await {
                Ok(_) => {
                    window().location().set_href("/dashboard").ok();
                }
                Err(e) => {
                    set_local_error.set(Some(e));
                }
            }
        });
    };

    // Handle registration
    let handle_register = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_local_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let first_name_val = first_name.get();
        let last_name_val = last_name.get();

        if email_val.is_empty() || password_val.is_empty() {
            set_local_error.set(Some("Email and password are required".to_string()));
            return;
        }

        if password_val.len() < 8 {
            set_local_error.set(Some("Password must be at least 8 characters".to_string()));
            return;
        }

        let first_opt = if first_name_val.is_empty() {
            None
        } else {
            Some(first_name_val)
        };

        let last_opt = if last_name_val.is_empty() {
            None
        } else {
            Some(last_name_val)
        };

        spawn_local(async move {
            match auth.register(email_val, password_val, first_opt, last_opt).await {
                Ok(_) => {
                    window().location().set_href("/dashboard").ok();
                }
                Err(e) => {
                    set_local_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8 p-8 bg-white rounded-2xl shadow-xl">
                <div>
                    <h2 class="text-center text-3xl font-extrabold text-gray-900">
                        {move || if mode.get() == "login" {
                            "Sign in to TaskForge"
                        } else {
                            "Create your account"
                        }}
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        {move || if mode.get() == "login" {
                            view! {
                                <span>
                                    "Don't have an account? "
                                    <button
                                        type="button"
                                        class="font-medium text-indigo-600 hover:text-indigo-500"
                                        on:click=move |_| {
                                            set_mode.set("register");
                                            set_local_error.set(None);
                                        }
                                    >
                                        "Sign up"
                                    </button>
                                </span>
                            }
                        } else {
                            view! {
                                <span>
                                    "Already have an account? "
                                    <button
                                        type="button"
                                        class="font-medium text-indigo-600 hover:text-indigo-500"
                                        on:click=move |_| {
                                            set_mode.set("login");
                                            set_local_error.set(None);
                                        }
                                    >
                                        "Sign in"
                                    </button>
                                </span>
                            }
                        }}
                    </p>
                </div>

                {move || {
                    let error = local_error.get().or_else(|| auth.error.get());
                    error.map(|err| {
                        view! {
                            <div class="rounded-md bg-red-50 p-4">
                                <div class="flex">
                                    <div class="ml-3">
                                        <h3 class="text-sm font-medium text-red-800">
                                            {err}
                                        </h3>
                                    </div>
                                </div>
                            </div>
                        }
                    })
                }}

                <Show
                    when=move || mode.get() == "login"
                    fallback=move || view! {
                        <form class="mt-8 space-y-6" on:submit=handle_register>
                            <div class="space-y-4">
                                <div>
                                    <label for="email" class="block text-sm font-medium text-gray-700">
                                        "Email address"
                                    </label>
                                    <input
                                        id="email"
                                        name="email"
                                        type="email"
                                        required
                                        class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="you@example.com"
                                        prop:value=move || email.get()
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label for="first-name" class="block text-sm font-medium text-gray-700">
                                        "First name (optional)"
                                    </label>
                                    <input
                                        id="first-name"
                                        name="first-name"
                                        type="text"
                                        class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="John"
                                        prop:value=move || first_name.get()
                                        on:input=move |ev| set_first_name.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label for="last-name" class="block text-sm font-medium text-gray-700">
                                        "Last name (optional)"
                                    </label>
                                    <input
                                        id="last-name"
                                        name="last-name"
                                        type="text"
                                        class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="Doe"
                                        prop:value=move || last_name.get()
                                        on:input=move |ev| set_last_name.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label for="password" class="block text-sm font-medium text-gray-700">
                                        "Password"
                                    </label>
                                    <input
                                        id="password"
                                        name="password"
                                        type="password"
                                        required
                                        class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="At least 8 characters"
                                        prop:value=move || password.get()
                                        on:input=move |ev| set_password.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>

                            <div>
                                <button
                                    type="submit"
                                    disabled=move || auth.is_loading.get()
                                    class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {move || if auth.is_loading.get() {
                                        "Creating account..."
                                    } else {
                                        "Sign up"
                                    }}
                                </button>
                            </div>
                        </form>
                    }
                >
                    <form class="mt-8 space-y-6" on:submit=handle_login>
                        <div class="space-y-4">
                            <div>
                                <label for="email" class="block text-sm font-medium text-gray-700">
                                    "Email address"
                                </label>
                                <input
                                    id="email"
                                    name="email"
                                    type="email"
                                    required
                                    class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                    placeholder="you@example.com"
                                    prop:value=move || email.get()
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label for="password" class="block text-sm font-medium text-gray-700">
                                    "Password"
                                </label>
                                <input
                                    id="password"
                                    name="password"
                                    type="password"
                                    required
                                    class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                    placeholder="Your password"
                                    prop:value=move || password.get()
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        <div>
                            <button
                                type="submit"
                                disabled=move || auth.is_loading.get()
                                class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                {move || if auth.is_loading.get() {
                                    "Signing in..."
                                } else {
                                    "Sign in"
                                }}
                            </button>
                        </div>
                    </form>
                </Show>
            </div>
        </div>
    }
}
