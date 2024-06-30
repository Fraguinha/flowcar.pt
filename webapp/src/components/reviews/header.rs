use leptos::*;

use crate::components::reviews::rating::Rating;

#[component]
pub fn ReviewsHeader(overall_rating: f32, total_reviews: i32) -> impl IntoView {
    view! {
        <div class="d-flex justify-content-between align-items-center mb-4 p-3 bg-light rounded">
            <div class="d-flex align-items-center">
                <div>
                    <h4 class="mb-0">{"Google Reviews"}</h4>
                    <div>
                        <Rating rating=overall_rating total_reviews=Some(total_reviews)/>
                    </div>
                </div>
            </div>
            <a href="" class="btn btn-primary">
                "Review us on Google"
            </a>
        </div>
    }
}
