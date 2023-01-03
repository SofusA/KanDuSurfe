use crate::models::forecast::ForeCastRoot;
use crate::models::forecast::ForecastProvider;
use crate::models::spot::Spot;
use crate::models::surfday::*;
use async_trait::async_trait;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use reqwest::header::USER_AGENT;

use super::get_surfdays::get_surfdays;

pub async fn get_response() -> impl IntoResponse {
    let yr_forecast: YrForecast = ForecastProvider::new();
    let response = get_forecast_from_provider(yr_forecast).await;

    // Ok(serialise_forecast_to_string(response))

    (StatusCode::CREATED, Json(response))
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
        YrForecast {}
    }
}

pub async fn get_forecast_from_provider(provider: impl ForecastProvider) -> Vec<SurfDayResponse> {
    let surf_days = get_surfdays(provider).await;
    surf_days.iter().map(|x| x.to_response()).collect()
}
