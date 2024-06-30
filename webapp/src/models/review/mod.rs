use crate::models::media::Media;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub image: Media,
    pub username: String,
    pub when: String,
    pub rating: f32,
    pub review: String,
}
