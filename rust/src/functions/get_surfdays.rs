use std::collections::HashSet;

use crate::functions::get_forecast::get_forecast;
use crate::functions::get_spots::get_spots;
use crate::models::spot::Direction;
use crate::models::surf_constants::SurfConstants;

use chrono::{DateTime, Timelike};

#[derive(Debug)]
pub struct SurfDay {
    pub day: String,
    pub spots: HashSet<String>,
}

pub fn get_surfdays() -> Vec<SurfDay> {
    let mut response = Vec::<SurfDay>::new();
    let spots = get_spots();
    let surf_constants = SurfConstants {
        start_hour: 8,
        stop_hour: 20,
        wind_speed: 7.5,
    };

    for spot in spots {
        let forecast = get_forecast(&spot);
        //let mut surf_days = HashSet::<String>::new();

        for timeserie in forecast.properties.timeseries {
            let datetime = DateTime::parse_from_rfc3339(&timeserie.time).expect("Error reading time format");

            let forecast_hour = datetime.hour();
            let forecast_data = timeserie.data.instant.details;

            if forecast_hour < surf_constants.start_hour || forecast_hour > surf_constants.stop_hour {
                continue;
            };
            if forecast_data.wind_speed < surf_constants.wind_speed {
                continue;
            };
            if !surfable_direction(&spot.directions, &forecast_data.wind_from_direction) {
                continue;
            };

            let day = datetime.format("%y/%m/%d").to_string();

            let matcher = response.iter_mut().find(|surf_day| surf_day.day == day);

            match matcher {
                Some(surf_day)=> {
                    surf_day.spots.insert(spot.name);

                    break;
                },
                None => {
                    let mut spot_name_hashset = HashSet::new();
                    spot_name_hashset.insert(spot.name.clone());

                    response.push(SurfDay{
                        day: day,
                        spots: spot_name_hashset
                    })
                }
            }
        }
    }

    response.sort_by_key(|x| x.day.clone());

    return response;
}

fn surfable_direction(allowed_directions: &Vec<Direction>, forecast_wind_direction: &f64) -> bool
{
    for direction in allowed_directions {
        let min = direction.minimum as f64;
        let max = direction.maximum as f64;

        if forecast_wind_direction.min(min).eq(forecast_wind_direction) && forecast_wind_direction.max(max).eq(forecast_wind_direction){
            return true;
        }   
    }
    return false;
}