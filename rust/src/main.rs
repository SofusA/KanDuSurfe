use std::collections::HashSet;

use itertools::Itertools;

use crate::{functions::get_surfdays::get_surfdays};
extern crate itertools;

mod models;
mod functions;

struct Response {
    surf: bool,
    text: String
}

fn main() {
    let response: Response;
    let surf_days = get_surfdays();

    println!("{:#?}", surf_days);

    match surf_days.first() {
        Some(surf_day) => response = Response{surf: true, text: format!("Du kan surfe d. {} på {}", surf_day.day, spots_to_string(&surf_day.spots))},
        None => response = Response{surf: false, text: "Du kan ikke surfe.".to_string()},
    }

    println!("{:#?}", response.text);
}

fn spots_to_string(spots: &HashSet<String>) -> String {
    return spots.iter().join(" og ");
}