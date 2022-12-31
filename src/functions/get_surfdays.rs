use chrono::{DateTime, Datelike, FixedOffset, Timelike};
use std::collections::HashSet;

use crate::functions::get_spots::get_spots;
use crate::models::date::{CompareDates, Date, DateSpan};
use crate::models::forecast::ForecastProvider;
use crate::models::spot::Direction;
use crate::models::surf_constants::SurfConstants;
use crate::models::surfday::SurfDay;

pub async fn get_surfdays(provider: impl ForecastProvider) -> Vec<SurfDay> {
    let mut response = Vec::<SurfDay>::new();
    let spots = get_spots();
    let surf_constants = SurfConstants {
        start_hour: 8,
        stop_hour: 20,
        wind_speed: 7.5,
        temperature: 5.0,
    };

    for spot in spots {
        let forecast = provider.get_forecast(&spot).await;

        let mut continuos_counter = 0;

        for timeserie in forecast.properties.timeseries {
            let date_time =
                DateTime::parse_from_rfc3339(&timeserie.time).expect("Error reading time format");

            let forecast_hour = date_time.hour();
            let forecast_data = timeserie.data.instant.details;

            if is_date_in_inactive_dates(&date_time, &spot.inactive_dates) {
                continue;
            };

            if forecast_hour < surf_constants.start_hour || forecast_hour > surf_constants.stop_hour
            {
                continue;
            };

            if forecast_data.air_temperature < surf_constants.temperature {
                continue;
            }

            if forecast_data.wind_speed < surf_constants.wind_speed {
                continue;
            };

            if !surfable_direction(&spot.directions, &forecast_data.wind_from_direction) {
                continue;
            };

            continuos_counter += 1;

            if continuos_counter < 3 {
                continue;
            }

            let matcher = response
                .iter_mut()
                .find(|surf_day| surf_day.day.date() == date_time.date());

            match matcher {
                Some(surf_day) => {
                    surf_day.spots.insert(spot.name.clone());
                }
                None => {
                    let mut spot_name_hashset = HashSet::new();
                    spot_name_hashset.insert(spot.name.clone());

                    response.push(SurfDay {
                        day: date_time,
                        spots: spot_name_hashset,
                    })
                }
            }
        }
    }

    response.sort_by_key(|x| x.day.clone());

    return response;
}

fn is_date_in_inactive_dates(
    date_time: &DateTime<FixedOffset>,
    inactive_dates: &Vec<DateSpan>,
) -> bool {
    for inactive_date in inactive_dates {
        let date = Date {
            day: date_time.day(),
            month: date_time.month(),
        };

        if inactive_date.is_between_dates(&date) {
            return true;
        }
    }

    return false;
}

fn surfable_direction(allowed_directions: &Vec<Direction>, forecast_wind_direction: &f32) -> bool {
    for direction in allowed_directions {
        let min = direction.minimum as f32;
        let max = direction.maximum as f32;

        if !is_lower(forecast_wind_direction, &min) && is_lower(forecast_wind_direction, &max) {
            return true;
        }
    }

    return false;
}

fn is_lower(first: &f32, second: &f32) -> bool {
    return first.min(*second).eq(first);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lower() {
        let low: f32 = 5.0;
        let high: f32 = 10.0;

        assert_eq!(is_lower(&low, &high), true);
    }

    #[test]
    fn test_surfable_direction_returns_true() {
        let allowed_dir: f32 = 100.0;
        let forecast_dir = vec![Direction {
            minimum: 50,
            maximum: 150,
        }];

        assert_eq!(surfable_direction(&forecast_dir, &allowed_dir), true);
    }

    #[test]
    fn test_surfable_direction_returns_false() {
        let allowed_dir: f32 = 150.0;
        let forecast_dir = vec![Direction {
            minimum: 50,
            maximum: 100,
        }];

        assert_eq!(surfable_direction(&forecast_dir, &allowed_dir), false);
    }

    #[test]
    fn test_surfable_direction_vec() {
        let allowed_dir: f32 = 15.0;
        let not_allowed_dir: f32 = 30.0;
        let forecast_dir = vec![
            Direction {
                minimum: 10,
                maximum: 20,
            },
            Direction {
                minimum: 50,
                maximum: 100,
            },
        ];

        assert_eq!(surfable_direction(&forecast_dir, &allowed_dir), true);
        assert_eq!(surfable_direction(&forecast_dir, &not_allowed_dir), false);
    }
}
