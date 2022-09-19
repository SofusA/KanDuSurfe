use chrono::Duration;
use chrono::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, convert::Infallible};

use super::get_surfdays::get_surfdays;

pub fn in_future_range(input_dt: DateTime<FixedOffset>, range_dur: Duration) -> bool {
    let utc_now_dt = Utc::now();

    return utc_now_dt < input_dt && input_dt <= utc_now_dt + range_dur;
}

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let response: Response;

    let surf_days = get_surfdays().await;

    let condition = surf_days.first().filter(|surf_day| in_future_range(surf_day.day, Duration::days(3)));

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

    let json_response = serde_json::to_string(&response).unwrap();

    return Ok(json_response);
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub msg: String,
    pub surfable: bool,
}
