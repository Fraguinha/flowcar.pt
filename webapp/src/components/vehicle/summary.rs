use leptos::*;

use crate::components::vehicle::feature::VehicleFeature;
use crate::models::vehicle::fuel::Fuel;
use crate::models::vehicle::Vehicle;

#[component]
pub fn VehicleSummary(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="row">
            <h5 class="col-12 card-title">{&vehicle.title}</h5>
            <div class="col-7">
                {match vehicle.fuel {
                    Fuel::Electric => {
                        view! {
                            <small class="text-secondary">
                                {format!("{} cv", vehicle.horsepower)}
                            </small>
                        }
                    }
                    Fuel::Diesel | Fuel::Gas | Fuel::Hybrid => {
                        view! {
                            <small class="text-secondary">
                                {format!(
                                    "{} cv · {} cm3",
                                    vehicle.horsepower,
                                    vehicle.displacement,
                                )}

                            </small>
                        }
                    }
                }}

            </div>
            <h5 class="col-5 card-title text-end">{format!("{}€", vehicle.price)}</h5>
        </div>

        <div class="mb-3"></div>
        <VehicleFeature
            icon=String::from("bi bi-calendar-date")
            text=format!("{}", vehicle.year)
            vertical=false
        />
        <VehicleFeature
            icon=String::from("bi bi-sign-turn-slight-right")
            text=format!("{} km", vehicle.mileage)
            vertical=false
        />
        <VehicleFeature
            icon=match vehicle.fuel {
                Fuel::Electric => String::from("bi bi-ev-station"),
                Fuel::Diesel => String::from("bi bi-fuel-pump-diesel"),
                Fuel::Gas | Fuel::Hybrid => String::from("bi bi-fuel-pump"),
            }

            text=format!("{}", vehicle.fuel)
            vertical=false
        />
        <VehicleFeature
            icon=String::from("bi bi-gear-wide")
            text=format!("{}", vehicle.transmission)
            vertical=false
        />
    }
}
