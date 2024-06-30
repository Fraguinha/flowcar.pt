use leptos::*;
use leptos_router::ActionForm;

use crate::auth::Login;

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="form-signin text-center">
            <ActionForm action=action>
                <img class="mb-4" src="/images/logo.svg" alt="" width="72" height="72"/>
                <h1 class="h3 mb-3 fw-normal">Flow Car Studio</h1>
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
                <div class="checkbox mb-3">
                    <input type="checkbox" name="remember" id="checkbox"/>
                    <label for="checkbox">Remember me</label>
                </div>
                <button class="w-100 btn btn-lg btn-primary" type="submit">
                    {"Log in"}
                </button>
            </ActionForm>
        </div>
    }
}
