use leptos::*;

use crate::components::auth::profile::Profile;
use crate::components::footer::Footer;
use crate::components::nav::NavBar;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div class="fullscreen">
            <NavBar/>
            <Profile/>
        </div>
        <Footer/>
    }
}
