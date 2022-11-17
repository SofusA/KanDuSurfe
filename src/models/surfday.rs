use std::collections::HashSet;
use chrono::{DateTime, FixedOffset};
use itertools::Itertools;
use serde::Serialize;

pub struct SurfDay {
    pub day: DateTime<FixedOffset>,
    pub spots: HashSet<String>,
}

#[derive(Debug, Serialize)]
pub struct SurfDayResponse {
    pub day: String,
    pub spots: Vec<String>,
}

pub trait ResponseType {
    fn to_response(&self) -> SurfDayResponse; 
}

impl ResponseType for SurfDay {
    fn to_response(&self) -> SurfDayResponse {
        let sorted_spot_names: Vec<String> = self.spots.clone().into_iter().sorted().collect_vec();
        
        let response = SurfDayResponse {
            day: self.day.format("%Y-%m-%d").to_string(),
            spots: sorted_spot_names,
        };
    
        return response;
    }
}