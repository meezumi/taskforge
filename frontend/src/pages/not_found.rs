use leptos::*;
use leptos_router::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-indigo-600">"404"</h1>
                <p class="mt-4 text-xl text-gray-600">"Page not found"</p>
                <A href="/" class="mt-6 inline-block px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700">
                    "Go Home"
                </A>
            </div>
        </div>
    }
}
