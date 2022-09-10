use std::{collections::HashSet, convert::Infallible};
use itertools::Itertools;

use super::get_surfdays::get_surfdays;

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let response: String;
    let surf_days = get_surfdays().await;

    print!("{:#?}", surf_days);

    match surf_days.first() {
        Some(surf_day) => {
            response = format!(
                "Du kan surfe d. {} på {}",
                surf_day.day.format("%Y-%m-%d"),
                spots_to_string(&surf_day.spots)
            )
        }
        None => response = "Du kan ikke surfe.".to_string(),
    }

    return Ok(response);
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}
