use leptos::*;
use uuid::Uuid;

use crate::components::use_auth_context;
use crate::services::organizations::{self, Organization};

#[derive(Copy, Clone)]
pub struct OrganizationContext {
    pub organizations: RwSignal<Vec<Organization>>,
    pub current_org: RwSignal<Option<Organization>>,
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl OrganizationContext {
    pub async fn load_organizations(self) {
        self.is_loading.set(true);
        self.error.set(None);

        match organizations::get_my_organizations().await {
            Ok(orgs) => {
                self.organizations.set(orgs.clone());
                // Set first org as current if none selected
                if self.current_org.get().is_none() && !orgs.is_empty() {
                    self.current_org.set(Some(orgs[0].clone()));
                }
            }
            Err(e) => {
                self.error.set(Some(format!("Failed to load organizations: {}", e)));
            }
        }

        self.is_loading.set(false);
    }

    pub async fn create_organization(
        self,
        name: String,
        slug: String,
        description: Option<String>,
    ) -> Result<Organization, String> {
        self.is_loading.set(true);
        self.error.set(None);

        let result = organizations::create_organization(name, slug, description).await;

        match result {
            Ok(org) => {
                // Add to list
                let mut orgs = self.organizations.get();
                orgs.push(org.clone());
                self.organizations.set(orgs);

                // Set as current
                self.current_org.set(Some(org.clone()));
                self.is_loading.set(false);
                Ok(org)
            }
            Err(e) => {
                let error_msg = format!("Failed to create organization: {}", e);
                self.error.set(Some(error_msg.clone()));
                self.is_loading.set(false);
                Err(error_msg)
            }
        }
    }

    pub fn set_current_organization(self, org: Organization) {
        self.current_org.set(Some(org));
    }

    pub fn set_current_organization_by_id(self, org_id: Uuid) {
        let orgs = self.organizations.get();
        if let Some(org) = orgs.iter().find(|o| o.id == org_id) {
            self.current_org.set(Some(org.clone()));
        }
    }
}

pub fn provide_organization_context() {
    let organizations = create_rw_signal(Vec::new());
    let current_org = create_rw_signal(None);
    let is_loading = create_rw_signal(false);
    let error = create_rw_signal(None);

    let context = OrganizationContext {
        organizations,
        current_org,
        is_loading,
        error,
    };

    provide_context(context);

    // Only load organizations when user is authenticated
    let auth = use_auth_context();
    create_effect(move |_| {
        if auth.user.get().is_some() {
            spawn_local(async move {
                context.load_organizations().await;
            });
        }
    });
}

pub fn use_organization_context() -> OrganizationContext {
    use_context::<OrganizationContext>().expect("OrganizationContext not provided")
}
