use leptos::*;

use crate::{components::reviews::rating::Rating, models::media::Media};

#[component]
pub fn ReviewCard(
    image: Media,
    username: String,
    when: String,
    rating: f32,
    review: String,
) -> impl IntoView {
    view! {
        <div class="card" style="max-width: 400px; height: 200px;">
            <div class="card-body d-flex flex-column">
                <div class="d-flex align-items-center mb-3">
                    <img
                        src=image.src
                        alt=image.alt
                        class="rounded-circle me-3"
                        style="max-width: 50px; height: 50px;"
                    />
                    <div>
                        <h5 class="card-title mb-0">{username}</h5>
                        <small class="text-muted">{when}</small>
                    </div>
                </div>
                <Rating rating=rating total_reviews=None/>
                <p class="card-text text-truncate" style="display: inline-block;">
                    {review}
                </p>
            </div>
        </div>
    }
}
