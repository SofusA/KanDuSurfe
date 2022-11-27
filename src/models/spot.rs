use super::date::DateSpan;

pub struct Spot {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub directions: Vec<Direction>,
    pub inactive_dates: Vec<DateSpan>
}

pub struct Direction {
    pub minimum: u32,
    pub maximum: u32
}