use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
            <nav class="bg-white shadow-sm">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16 items-center">
                        <div class="flex items-center">
                            <h1 class="text-2xl font-bold text-indigo-600">"TaskForge"</h1>
                        </div>
                        <div class="flex space-x-4">
                            <A href="/login" class="px-4 py-2 text-indigo-600 hover:text-indigo-800">
                                "Login"
                            </A>
                            <button class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700">
                                "Get Started"
                            </button>
                        </div>
                    </div>
                </div>
            </nav>

            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="text-center">
                    <h1 class="text-5xl font-extrabold text-gray-900 sm:text-6xl">
                        "Manage Projects"
                        <span class="text-indigo-600">" Efficiently"</span>
                    </h1>
                    <p class="mt-6 text-xl text-gray-600 max-w-3xl mx-auto">
                        "A modern, multi-tenant SaaS platform built with Rust and WebAssembly. 
                        Collaborate with your team, track progress, and deliver projects on time."
                    </p>
                    <div class="mt-10 flex justify-center gap-4">
                        <button class="px-8 py-3 bg-indigo-600 text-white text-lg font-medium rounded-lg hover:bg-indigo-700 shadow-lg">
                            "Start Free Trial"
                        </button>
                        <button class="px-8 py-3 bg-white text-indigo-600 text-lg font-medium rounded-lg border-2 border-indigo-600 hover:bg-indigo-50">
                            "View Demo"
                        </button>
                    </div>
                </div>

                <div class="mt-20 grid md:grid-cols-3 gap-8">
                    <FeatureCard
                        icon="🚀"
                        title="Fast & Efficient"
                        description="Built with Rust for maximum performance and reliability"
                    />
                    <FeatureCard
                        icon="🔒"
                        title="Secure Multi-Tenancy"
                        description="Complete data isolation with role-based access control"
                    />
                    <FeatureCard
                        icon="⚡"
                        title="Real-Time Updates"
                        description="WebSocket-based collaboration in real-time"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-xl shadow-md hover:shadow-xl transition-shadow">
            <div class="text-4xl mb-4">{icon}</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}
