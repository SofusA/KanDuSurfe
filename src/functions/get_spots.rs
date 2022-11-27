use crate::models::spot::{Direction, Spot};
use crate::models::date::{Date, DateSpan};

pub fn get_spots() -> Vec<Spot> {
    let spots = vec![Spot {
        name: "Sydvestpynten".to_string(),
        latitude: 55.565231,
        longitude: 12.558465,
        directions: vec![Direction {minimum: 180, maximum: 315}],
        inactive_dates: vec![]
    },
    Spot {
        name: "Amager Strandpark".to_string(),
        latitude: 55.653506,
        longitude: 12.645545,
        directions: vec![Direction {minimum: 45, maximum: 157}],
        inactive_dates: vec![]
    },
    Spot {
        name: "Lynæs".to_string(),
        latitude: 55.942500,
        longitude: 11.862056,
        directions: vec![Direction {minimum: 135, maximum: 315}],
        inactive_dates: vec![]
    },
    Spot {
        name: "Poppelvej".to_string(),
        latitude: 55.559259,
        longitude: 12.612591,
        directions: vec![Direction {minimum: 80, maximum: 180}],
        inactive_dates: vec![DateSpan {start_date: Date {day: 1, month: 11}, end_date: Date {day: 15, month: 7} }]
    },
    Spot {
        name: "Farø".to_string(),
        latitude: 54.942552,
        longitude: 12.001952,
        directions: vec![Direction {minimum: 225, maximum: 337}, Direction {minimum: 45, maximum: 135}],
        inactive_dates: vec![]
    }];

    return spots;
}