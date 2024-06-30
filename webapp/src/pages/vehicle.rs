use leptos::*;

use crate::components::footer::Footer;
use crate::components::nav::NavBar;
use crate::components::vehicle::Vehicle;

#[component]
pub fn VehiclePage() -> impl IntoView {
    view! {
        <NavBar/>
        <Vehicle/>
        <Footer/>
    }
}
