use leptos::*;

use crate::components::footer::Footer;
use crate::components::nav::NavBar;
use crate::components::reviews::Reviews;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="fullscreen">
            <NavBar/>
        </div>
        <Reviews/>
        <Footer/>
    }
}
