use leptos::*;

use crate::components::vehicle::summary::VehicleSummary;
use crate::models::vehicle::Vehicle;

#[component]
pub fn VehicleCard(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="card mb-3">
            <a href=format!("/vehicle/{}", vehicle.id)>
                <img src=&vehicle.images[0].src alt=&vehicle.images[0].alt class="img-fluid"/>
            </a>
            <div class="card-body">
                <VehicleSummary vehicle=vehicle/>
            </div>
        </div>
    }
}
