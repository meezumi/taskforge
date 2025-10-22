use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod services;

use pages::{
    dashboard::Dashboard, home::Home, login::Login, not_found::NotFound,
    organization_detail::OrganizationDetail, organizations::Organizations,
};
use components::{provide_auth_context, provide_organization_context};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    // Provide auth context to the entire app
    let _auth = provide_auth_context();
    
    // Provide organization context
    let _org = provide_organization_context();

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
                    <Route path="/organizations" view=Organizations/>
                    <Route path="/organizations/:org_id" view=OrganizationDetail/>
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
