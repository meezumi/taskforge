use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod services;

use pages::{home::Home, login::Login, dashboard::Dashboard, not_found::NotFound};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/taskforge-frontend.css"/>
        <Title text="TaskForge - Project Management"/>
        <Meta name="description" content="Modern multi-tenant project management platform"/>
        
        <Router>
            <main class="min-h-screen bg-gray-50">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/login" view=Login/>
                    <Route path="/dashboard" view=Dashboard/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    log::info!("🚀 TaskForge Frontend starting...");
    
    mount_to_body(|| view! { <App/> })
}
