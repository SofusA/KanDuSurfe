use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Duration;
use itertools::Itertools;
use reqwest::header::USER_AGENT;

use std::{collections::HashSet, convert::Infallible};

use crate::models::forecast::ForeCastRoot;
use crate::models::forecast::ForecastProvider;
use crate::models::response::Response;
use crate::models::spot::Spot;

use super::get_surfdays::get_surfdays;

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let yr_forecast: YrForecast = ForecastProvider::new();
    let response = get_forecast_from_provider(yr_forecast).await;
    let json_response = serde_json::to_string(&response).expect("Failed serialising json");

    return Ok(json_response);
}

struct YrForecast {}

#[async_trait]
impl ForecastProvider for YrForecast {
    async fn get_forecast(&self, spot: &Spot) -> ForeCastRoot {
        let forecast_root = reqwest::Client::new()
            .get(format!(
                "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
                spot.latitude, spot.longitude
            ))
            .header(USER_AGENT, "KanDuSurfe")
            .send()
            .await
            .expect("Failed recieving forecast")
            .json()
            .await
            .expect("Failed deserialise json");

        return forecast_root;
    }

    fn new() -> YrForecast {
        return YrForecast {};
    }
}

pub async fn get_forecast_from_provider(provider: impl ForecastProvider) -> Response {
    let response: Response;

    let surf_days = get_surfdays(provider).await;
    let condition = surf_days
        .first()
        .filter(|surf_day| in_future_range(surf_day.day, Duration::days(3)));

    match condition {
        Some(surf_day) => {
            response = Response {
                msg: format!(
                    "Du kan surfe d. {} på {}",
                    surf_day.day.format("%Y-%m-%d"),
                    spots_to_string(&surf_day.spots)
                ),
                surfable: true,
            }
        }
        None => {
            response = Response {
                msg: "Du kan ikke surfe.".to_string(),
                surfable: false,
            }
        }
    }

    return response;
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}

fn in_future_range(input_dt: DateTime<FixedOffset>, range_dur: Duration) -> bool {
    let utc_now_dt = Utc::now();

    return utc_now_dt < input_dt && input_dt <= utc_now_dt + range_dur;
}