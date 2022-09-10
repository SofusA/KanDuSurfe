use reqwest::header::USER_AGENT;

use crate::models::{spot::Spot, forecast::ForeCastRoot};

async fn get_forecast_response(spot: &Spot) -> Result<ForeCastRoot, reqwest::Error> {
    let forecast_root: ForeCastRoot = reqwest::Client::new()
        .get(format!("https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}", spot.latitude, spot.longitude))
        .header(USER_AGENT, "KanDuSurfe")
        .send()
        .await?
        .json()
        .await?;

    Ok(forecast_root)
}

pub async fn get_forecast(spot: &Spot) -> ForeCastRoot {
    let forecast = get_forecast_response(spot).await.expect("Failed recieving forecast");
    return forecast;
}