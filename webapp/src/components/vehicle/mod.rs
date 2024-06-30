pub mod card;
pub mod details;
pub mod feature;
pub mod gallery;
pub mod list;
pub mod search;
pub mod summary;

use leptos::*;
use leptos_router::{use_params_map, use_query_map};

use crate::components::vehicle::details::VehicleDetails;
use crate::components::vehicle::gallery::VehicleGallery;
use crate::components::vehicle::summary::VehicleSummary;
use crate::models::vehicle::model::Model;
use crate::models::vehicle::Vehicle;

#[server]
pub async fn get_vehicle(id: Option<String>) -> Result<(Vehicle, Model), ServerFnError> {
    use crate::database;

    let pool = database::db().await.unwrap();

    let vehicle = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles WHERE id = $1::uuid")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let model = sqlx::query_as::<_, Model>("SELECT * FROM models WHERE id = $1::uuid")
        .bind(vehicle.model)
        .fetch_one(&pool)
        .await?;

    Ok((vehicle, model))
}

#[component]
pub fn Vehicle() -> impl IntoView {
    let params = use_params_map();
    let _query = use_query_map();

    #[derive(Clone, PartialEq)]
    struct QueryInfo {
        id: Option<String>,
    }

    let query_info = move || {
        with!(|params, _query| QueryInfo {
            id: params.get("id").cloned(),
        })
    };

    let vehicle_resource = create_resource(query_info, move |info| get_vehicle(info.id));

    view! {
        <Suspense>
            {move || match vehicle_resource.get() {
                Some(Ok((vehicle, model))) => {
                    view! {
                        <>
                            <div class="container-lg conditional-shadow my-lg-5">
                                <div class="row">
                                    {match vehicle.video.src.is_empty() {
                                        true => {
                                            view! {
                                                <>
                                                    <div class="col-12 col-lg-6 p-0">
                                                        <div class="ratio ratio-1x1">
                                                            <img src=&vehicle.images[0].src alt=&vehicle.images[0].alt/>
                                                        </div>
                                                    </div>
                                                    <div class="col-12 col-lg-6 p-4">
                                                        <VehicleSummary vehicle=vehicle.clone()/>
                                                        <VehicleDetails
                                                            vehicle=vehicle.clone()
                                                            model=model.clone()
                                                        />
                                                    </div>
                                                </>
                                            }
                                        }
                                        false => {
                                            view! {
                                                <>
                                                    <div class="col-12 col-lg-4 col-md-6 p-0">
                                                        <div class="ratio ratio-9x16">
                                                            <video
                                                                src=&vehicle.video.src
                                                                alt=&vehicle.video.alt
                                                                autoplay
                                                                playsinline
                                                                loop
                                                                muted
                                                                controls
                                                            ></video>
                                                        </div>
                                                    </div>
                                                    <div class="col-12 col-lg-8 col-md-6 p-4">
                                                        <VehicleSummary vehicle=vehicle.clone()/>
                                                        <VehicleDetails
                                                            vehicle=vehicle.clone()
                                                            model=model.clone()
                                                        />
                                                    </div>
                                                </>
                                            }
                                        }
                                    }}

                                </div>
                            </div>
                            <VehicleGallery vehicle=vehicle.clone()/>
                        </>
                    }
                }
                Some(_) | None => view! { <>{}</> },
            }}

        </Suspense>
    }
}
