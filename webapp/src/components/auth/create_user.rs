use leptos::*;
use leptos_router::ActionForm;

use crate::auth::CreateUser;

#[component]
pub fn CreateUser(action: Action<CreateUser, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="form-signin text-center">
            <ActionForm action=action>
                <h1 class="h3 mb-3 fw-normal">Create User</h1>
                <div class="form-floating">
                    <input
                        type="text"
                        name="username"
                        class="form-control"
                        id="username"
                        placeholder="Username"
                        maxlength="32"
                    />
                    <label for="username">Username</label>
                </div>
                <div class="form-floating">
                    <input
                        type="password"
                        name="password"
                        class="form-control"
                        id="password"
                        placeholder="Password"
                    />
                    <label for="password">Password</label>
                </div>
                <div class="form-floating">
                    <input
                        type="password"
                        name="password_confirmation"
                        class="form-control"
                        id="password_confirmation"
                        placeholder="Password confirmation"
                    />
                    <label for="password_confirmation">Password confirmation</label>
                </div>
                <button class="w-100 btn btn-lg btn-primary" type="submit">
                    {"Create User"}
                </button>
            </ActionForm>
        </div>
    }
}
