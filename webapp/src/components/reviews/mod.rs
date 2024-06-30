pub mod card;
pub mod header;
pub mod rating;

use leptos::*;

use card::ReviewCard;
use header::ReviewsHeader;

use crate::models::{media::Media, review::Review};

#[component]
pub fn Reviews() -> impl IntoView {
    let reviews = vec![
        Review {
            image: Media {
                src: String::from(""),
                alt: String::from(""),
            },
            username: String::from("John Doe"),
            when: String::from("2 days ago"),
            rating: 5.0,
            review: String::from("Awesome! really enjoyed the service!"),
        },
        Review {
            image: Media {
                src: String::from(""),
                alt: String::from(""),
            },
            username: String::from("Jane Doe"),
            when: String::from("5 days ago"),
            rating: 5.0,
            review: String::from("Great service! will definitly come back!"),
        },
    ];

    view! {
        <div class="container">
            <ReviewsHeader overall_rating=4.8 total_reviews=42/>
            <div class="row">
                {reviews
                    .iter()
                    .map(|review| {
                        view! {
                            <div class="col-md-4 mb-4">
                                <ReviewCard
                                    image=review.image.clone()
                                    username=review.username.clone()
                                    when=review.when.clone()
                                    rating=review.rating
                                    review=review.review.clone()
                                />
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
