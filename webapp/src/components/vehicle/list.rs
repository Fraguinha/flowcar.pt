use leptos::*;

use crate::components::error::Error;
use crate::components::vehicle::card::VehicleCard;
use crate::models::vehicle::Vehicle;

#[component]
pub fn VehicleList(vehicles: Vec<Vehicle>) -> impl IntoView {
    if vehicles.is_empty() {
        view! {
            <>
                <Error
                    title=String::from("No Results")
                    subtitle=String::from("Please try searching for something different")
                    href=String::from("/")
                    text=String::from("Back to Home Page")
                />
            </>
        }
    } else {
        view! {
            <>
                <div class="container-fluid container-lg my-3">
                    <div class="row row-cols-1 row-cols-lg-3 g-3">
                        {vehicles
                            .iter()
                            .map(|vehicle| {
                                view! {
                                    <div class="col">
                                        <VehicleCard vehicle=vehicle.clone()/>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </>
        }
    }
}
