use crate::models::forecast::ForeCastRoot;
use crate::models::forecast::ForecastProvider;
use crate::models::spot::Spot;
use crate::models::surfday::*;
use anyhow::Result;
use async_trait::async_trait;
use axum::{http::StatusCode, response::IntoResponse};
use reqwest::header::USER_AGENT;

use super::get_surfdays::get_surfdays;

pub async fn get_response() -> impl IntoResponse {
    match get_serialized_forecast().await {
        Ok(res) => (StatusCode::OK, serde_json::to_string(&res).unwrap()),
        Err(err) => (StatusCode::BAD_GATEWAY, err.to_string()),
    }
}

async fn get_serialized_forecast() -> Result<String> {
    let yr_forecast: YrForecast = ForecastProvider::new();
    let response = get_forecast_from_provider(yr_forecast).await?;
    let serialized_response = serde_json::to_string(&response)?;

    Ok(serialized_response)
}

struct YrForecast {}

#[async_trait]
impl ForecastProvider for YrForecast {
    async fn get_forecast(&self, spot: &Spot) -> Result<ForeCastRoot> {
        let forecast_root = reqwest::Client::new()
            .get(format!(
                "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
                spot.latitude, spot.longitude
            ))
            .header(USER_AGENT, "KanDuSurfe")
            .send()
            .await?
            .json()
            .await?;

        return Ok(forecast_root);
    }

    fn new() -> YrForecast {
        YrForecast {}
    }
}

pub async fn get_forecast_from_provider(
    provider: impl ForecastProvider,
) -> Result<Vec<SurfDayResponse>> {
    let surf_days = get_surfdays(provider).await?;
    let surf_day_response = surf_days.iter().map(|x| x.to_response()).collect();

    Ok(surf_day_response)
}
