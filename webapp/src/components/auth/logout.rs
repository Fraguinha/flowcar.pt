use leptos::*;
use leptos_router::ActionForm;

use crate::auth::Logout;

#[component]
pub fn Logout(action: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="form-signin text-center">
            <ActionForm action=action>
                <button class="btn btn-lg btn-primary" type="submit">
                    {"Log out"}
                </button>
            </ActionForm>
        </div>
    }
}
