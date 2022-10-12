use std::collections::HashSet;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

pub struct SurfDay {
    pub day: DateTime<FixedOffset>,
    pub spots: HashSet<String>,
}

#[derive(Debug, Serialize)]
pub struct SurfDayResponse {
    pub day: String,
    pub spots: HashSet<String>,
}

pub fn convert_surf_day_response(surf_day: &SurfDay) -> SurfDayResponse {
    let response = SurfDayResponse {
        day: surf_day.day.format("%Y-%m-%d").to_string(),
        spots: surf_day.spots.clone(),
    };

    return response;

}