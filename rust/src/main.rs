use std::collections::HashSet;
use itertools::Itertools;

use crate::functions::get_surfdays::get_surfdays;

mod models;
mod functions;

struct Response {
    text: String
}

fn main() {
    let response: Response;
    let surf_days = get_surfdays();

    println!("{:#?}", surf_days);

    match surf_days.first() {
        Some(surf_day) => response = Response{text: format!("Du kan surfe d. {} på {}", surf_day.day, spots_to_string(&surf_day.spots))},
        None => response = Response{text: "Du kan ikke surfe.".to_string()},
    }

    println!("{:#?}", response.text);
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}