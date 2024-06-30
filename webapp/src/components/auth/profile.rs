use leptos::*;

use crate::components::auth::login::Login;
use crate::components::auth::logout::Logout;

use crate::auth::{get_user, Login, Logout};

#[component]
pub fn Profile() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();

    let user = create_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| get_user(),
    );

    view! {
        <Transition fallback=move || {
            view! {}
        }>
            {move || {
                user.get()
                    .map(|user| match user {
                        Ok(None) => view! { <Login action=login/> }.into_view(),
                        Ok(Some(_)) => {
                            view! {
                                <span>
                                    <Logout action=logout/>
                                </span>
                            }
                                .into_view()
                        }
                        Err(e) => view! { <span>{format!("Login error: {}", e)}</span> }.into_view(),
                    })
            }}

        </Transition>
    }
}
