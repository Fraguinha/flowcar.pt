use leptos::*;
use leptos_router::{use_params_map, use_query_map};

use crate::components::pagination::Pagination;
use crate::components::vehicle::list::VehicleList;
use crate::models::vehicle::Vehicle;

#[server]
pub async fn get_vehicles(
    page: Option<i32>,
    category: Option<String>,
    fuel: Option<String>,
) -> Result<Vec<Vehicle>, ServerFnError> {
    use crate::database;

    let pool = database::db().await.unwrap();

    let query = "SELECT * FROM vehicles";
    let pagination = "LIMIT $1 OFFSET $2";

    let filter = match (category, fuel) {
        (Some(category), _) => Some((
            "WHERE category = $3::vehicle_category",
            category.to_lowercase(),
        )),
        (_, Some(fuel)) => Some(("WHERE fuel = $3::vehicle_fuel", fuel.to_lowercase())),
        (_, _) => None,
    };

    let vehicles = match filter {
        Some((filter, value)) => {
            sqlx::query_as::<_, Vehicle>(format!("{query} {filter} {pagination}").as_str())
                .bind(database::ROW_LIMIT)
                .bind(page.map(|number| database::ROW_LIMIT * (number - 1)))
                .bind(value)
                .fetch_all(&pool)
                .await?
        }
        None => {
            sqlx::query_as::<_, Vehicle>(format!("{query} {pagination}").as_str())
                .bind(database::ROW_LIMIT)
                .bind(page.map(|number| database::ROW_LIMIT * (number - 1)))
                .fetch_all(&pool)
                .await?
        }
    };

    Ok(vehicles)
}

#[component]
pub fn VehicleSearch() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    #[derive(Clone, PartialEq)]
    struct SearchOptions {
        page: Option<i32>,
        category: Option<String>,
        fuel: Option<String>,
    }

    let search_options = move || {
        with!(|params, query| SearchOptions {
            page: params
                .get("page")
                .cloned()
                .map(|string| string.parse::<i32>().unwrap_or(1)),
            category: query.get("category").cloned(),
            fuel: query.get("fuel").cloned(),
        })
    };

    let vehicle_resource = create_resource(search_options, move |option| {
        get_vehicles(option.page, option.category, option.fuel)
    });

    view! {
        <Suspense>
            {move || match vehicle_resource.get() {
                Some(Ok(vehicles)) => {
                    let total_pages = (vehicles.len() as f32 / 12.0).ceil() as i32;
                    let current_page = search_options().page.unwrap_or(1);
                    let query = match (search_options().category, search_options().fuel) {
                        (Some(category), _) => format!("?category={}", category),
                        (_, Some(fuel)) => format!("?fuel={}", fuel),
                        (_, _) => String::from(""),
                    };
                    view! {
                        <>
                            <VehicleList vehicles/>
                            <Pagination current_page=current_page total_pages=total_pages query/>
                        </>
                    }
                }
                Some(_) | None => view! { <>{}</> },
            }}

        </Suspense>
    }
}
