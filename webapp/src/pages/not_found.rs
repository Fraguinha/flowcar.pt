use leptos::*;

use crate::components::error::Error;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Error
            title=String::from("404")
            subtitle=String::from("Not Found")
            href=String::from("/")
            text=String::from("Back to Home Page")
        />
    }
}
