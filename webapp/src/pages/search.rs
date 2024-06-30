use leptos::*;

use crate::components::footer::Footer;
use crate::components::nav::NavBar;
use crate::components::vehicle::search::VehicleSearch;

#[component]
pub fn SearchPage() -> impl IntoView {
    view! {
        <NavBar/>
        <VehicleSearch/>
        <Footer/>
    }
}
