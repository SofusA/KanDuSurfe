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

#[derive(Debug)]
pub struct DateSpan {
    pub start_date: Date,
    pub end_date: Date
}

#[derive(Clone, Debug)]
pub struct Date {
    pub day: u32, 
    pub month: u32
}

pub trait CompareDates {
    fn is_between_dates(&self, input: &Date) -> bool;
}

impl CompareDates for DateSpan {
    fn is_between_dates(&self, input: &Date) -> bool {  
        if self.start_date.month > self.end_date.month{
            let first_half = DateSpan { start_date: self.start_date.clone(), end_date: Date { day: 31, month: 12 } };
            let second_half = DateSpan { start_date: Date { day: 1, month: 1}, end_date: self.end_date.clone() };

            let first_check = first_half.is_between_dates(input);
            let second_check = second_half.is_between_dates(input);

            if first_check || second_check {
                return true;
            };
            return false;
        }

        if self.start_date.month == input.month || self.end_date.month == input.month {
            if self.start_date.day > input.day {
                return false
            }
    
            if self.end_date.day < input.day {
                return false
            }

            return true;
        } 

        if self.start_date.month > input.month {
            return false;
        } 

        if self.end_date.month < input.month {
            return false;
        }
        
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_between_dates_same_year() {
        let test_dates_same_year = DateSpan { start_date: Date { day: 1, month: 1 }, end_date: Date { day: 10, month: 4 } };

        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 2, month: 2 }), true);
        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 9, month: 4 }), true);
        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 10, month: 4 }), true);
        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 1, month: 1 }), true);

        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 2, month: 5 }), false);
        assert_eq!(test_dates_same_year.is_between_dates(&Date { day: 11, month: 4 }), false);
    }

    #[test]
    fn is_between_dates_multi_year() {
        let test_dates_multi_year = DateSpan { start_date: Date { day: 1, month: 11 }, end_date: Date { day: 15, month: 7 } };

        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 10, month: 2 }), true);
        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 2, month: 12 }), true);
        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 2, month: 5 }), true);
        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 20, month: 8 }), false);
        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 20, month: 2 }), true);
    }

    #[test]
    fn is_between_dates_higher_day() {
        let test_dates_multi_year = DateSpan { start_date: Date { day: 1, month: 1 }, end_date: Date { day: 15, month: 7 } };

        assert_eq!(test_dates_multi_year.is_between_dates(&Date { day: 20, month: 2 }), true);
    }
}