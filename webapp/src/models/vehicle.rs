use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::postgres::PgRow;
#[cfg(feature = "ssr")]
use sqlx::{Error, FromRow, Row};
use uuid::Uuid;

use crate::models::category::Category;
use crate::models::fuel::Fuel;
use crate::models::media::Media;
use crate::models::transmission::Transmission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub title: String,
    pub image: Media,
    pub video: Media,
    pub category: Category,
    pub fuel: Fuel,
    pub transmission: Transmission,
    pub price: i32,
    pub price_monthly: i32,
    pub year: i32,
    pub mileage: i32,
    pub horsepower: i32,
    pub displacement: i32,
    pub extra: Vec<String>,
    pub model: Uuid,
}

#[cfg(feature = "ssr")]
impl<'r> FromRow<'r, PgRow> for Vehicle {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let image_json: serde_json::Value = row.try_get("image")?;
        let video_json: serde_json::Value = row.try_get("video")?;

        let image = Media {
            src: image_json["src"].as_str().unwrap().to_string(),
            alt: image_json["alt"].as_str().unwrap().to_string(),
        };

        let video = Media {
            src: video_json["src"].as_str().unwrap().to_string(),
            alt: video_json["alt"].as_str().unwrap().to_string(),
        };

        Ok(Vehicle {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            image,
            video,
            category: row.try_get("category")?,
            fuel: row.try_get("fuel")?,
            transmission: row.try_get("transmission")?,
            price: row.try_get("price")?,
            price_monthly: row.try_get("price_monthly")?,
            year: row.try_get("year")?,
            mileage: row.try_get("mileage")?,
            horsepower: row.try_get("horsepower")?,
            displacement: row.try_get("displacement")?,
            extra: row.try_get("extra")?,
            model: row.try_get("model")?,
        })
    }
}
