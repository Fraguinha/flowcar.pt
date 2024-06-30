use leptos::*;

#[component]
pub fn Rating(rating: f32, total_reviews: Option<i32>) -> impl IntoView {
    let full_stars = rating.floor() as i32;
    let has_half_star = (rating - full_stars as f32) >= 0.5;
    let empty_stars = 5 - full_stars - if has_half_star { 1 } else { 0 };

    view! {
        <div class="mb-3">
            <span class="text-warning">
                {(0..full_stars)
                    .map(|_| {
                        view! { <i class="bi bi-star-fill"></i> }
                    })
                    .collect_view()}
                {if has_half_star {
                    view! {
                        <>
                            <i class="bi bi-star-half"></i>
                        </>
                    }
                } else {
                    view! { <>{}</> }
                }}
                {(0..empty_stars)
                    .map(|_| {
                        view! { <i class="bi bi-star"></i> }
                    })
                    .collect_view()} <small class="text-muted">{format!(" {}/5", rating)}</small>
            </span>
            <p>
                {match total_reviews {
                    Some(number) => {
                        view! {
                            <>
                                <small class="text-muted">{format!("({} reviews)", number)}</small>
                            </>
                        }
                    }
                    None => {
                        view! { <>{}</> }
                    }
                }}

            </p>
        </div>
    }
}
