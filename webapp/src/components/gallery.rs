use leptos::*;

use crate::models::vehicle::Vehicle;

#[component]
pub fn Gallery(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="bg-light">
            <div class="row offset-lg-3 col-lg-6 offset-1 py-5">
                <h3 class="card-title">
                    {"Gallery"}
                </h3>
            </div>
            <div class="row offset-lg-3 col-lg-6 pb-5">
                <div id="vehicleCarousel" class="carousel slide p-0" data-bs-ride="carousel">
                    <div class="carousel-inner">
                        <div class="carousel-item active">
                            <img src=&vehicle.image.src alt=&vehicle.image.alt class="d-block w-100"/>
                        </div>
                        <div class="carousel-item">
                            <img src=&vehicle.image.src alt=&vehicle.image.alt class="d-block w-100"/>
                        </div>
                        <div class="carousel-item">
                            <img src=&vehicle.image.src alt=&vehicle.image.alt class="d-block w-100"/>
                        </div>
                        <div class="carousel-item">
                            <img src=&vehicle.image.src alt=&vehicle.image.alt class="d-block w-100"/>
                        </div>
                    </div>
                    <button class="carousel-control-prev" type="button" data-bs-target="#vehicleCarousel" data-bs-slide="prev">
                        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Previous</span>
                    </button>
                    <button class="carousel-control-next" type="button" data-bs-target="#vehicleCarousel" data-bs-slide="next">
                        <span class="carousel-control-next-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Next</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
