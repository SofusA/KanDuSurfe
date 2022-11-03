use async_trait::async_trait;
use reqwest::header::USER_AGENT;
use std::convert::Infallible;
use crate::models::forecast::ForeCastRoot;
use crate::models::forecast::ForecastProvider;
use crate::models::spot::Spot;
use crate::models::surfday::SurfDayResponse;
use crate::models::surfday::convert_surf_day_response;

use super::get_surfdays::get_surfdays;

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let yr_forecast: YrForecast = ForecastProvider::new();
    let response = get_forecast_from_provider(yr_forecast).await;

    return Ok(serialise_forecast_to_string(response));
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

pub async fn get_forecast_from_provider(provider: impl ForecastProvider) -> Vec<SurfDayResponse> {
    let surf_days = get_surfdays(provider).await;
    surf_days.iter().map(|x| convert_surf_day_response(x)).collect()
}

fn serialise_forecast_to_string(forecast: Vec<SurfDayResponse>) -> String {
    if forecast.is_empty() {
        return "No surf".to_string();
    }

    serde_json::to_string(&forecast).expect("Failed serialising json")
}

#[test]
fn serialise_forecast_to_string_test() {
    let empty_forecast = Vec::<SurfDayResponse>::new();

    let response = serialise_forecast_to_string(empty_forecast);

    assert_eq!(response, "No surf");
}