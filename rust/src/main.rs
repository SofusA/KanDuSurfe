use std::collections::HashSet;
use itertools::Itertools;

use crate::functions::get_surfdays::get_surfdays;

mod models;
mod functions;

fn main() {
    let response: String;
    let surf_days = get_surfdays();

    match surf_days.first() {
        Some(surf_day) => response = format!("Du kan surfe d. {} på {}", surf_day.day, spots_to_string(&surf_day.spots)),
        None => response = "Du kan ikke surfe.".to_string(),
    }

    println!("{:#?}", response);
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}