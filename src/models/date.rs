#[derive(Debug)]
pub struct DateSpan {
    pub start_date: Date,
    pub end_date: Date,
}

#[derive(Clone, Debug)]
pub struct Date {
    pub day: u32,
    pub month: u32,
}

pub trait CompareDates {
    fn is_between_dates(&self, input: &Date) -> bool;
}

impl CompareDates for DateSpan {
    fn is_between_dates(&self, input: &Date) -> bool {
        if self.start_date.month > self.end_date.month {
            let first_half = DateSpan {
                start_date: self.start_date.clone(),
                end_date: Date { day: 31, month: 12 },
            };
            let second_half = DateSpan {
                start_date: Date { day: 1, month: 1 },
                end_date: self.end_date.clone(),
            };

            let first_check = first_half.is_between_dates(input);
            let second_check = second_half.is_between_dates(input);

            if first_check || second_check {
                return true;
            };
            return false;
        }

        if self.start_date.month == input.month || self.end_date.month == input.month {
            if self.start_date.day > input.day {
                return false;
            }

            if self.end_date.day < input.day {
                return false;
            }

            return true;
        }

        if self.start_date.month > input.month {
            return false;
        }

        if self.end_date.month < input.month {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_between_dates_same_year() {
        let test_dates_same_year = DateSpan {
            start_date: Date { day: 1, month: 1 },
            end_date: Date { day: 10, month: 4 },
        };

        assert!(test_dates_same_year.is_between_dates(&Date { day: 2, month: 2 }));
        assert!(test_dates_same_year.is_between_dates(&Date { day: 9, month: 4 }));
        assert!(test_dates_same_year.is_between_dates(&Date { day: 10, month: 4 }));
        assert!(test_dates_same_year.is_between_dates(&Date { day: 1, month: 1 }));

        assert!(!test_dates_same_year.is_between_dates(&Date { day: 2, month: 5 }));
        assert!(!test_dates_same_year.is_between_dates(&Date { day: 11, month: 4 }));
    }

    #[test]
    fn is_between_dates_multi_year() {
        let test_dates_multi_year = DateSpan {
            start_date: Date { day: 1, month: 11 },
            end_date: Date { day: 15, month: 7 },
        };

        assert!(test_dates_multi_year.is_between_dates(&Date { day: 10, month: 2 }));
        assert!(test_dates_multi_year.is_between_dates(&Date { day: 2, month: 12 }));
        assert!(test_dates_multi_year.is_between_dates(&Date { day: 2, month: 5 }));
        assert!(!test_dates_multi_year.is_between_dates(&Date { day: 20, month: 8 }));
        assert!(test_dates_multi_year.is_between_dates(&Date { day: 20, month: 2 }));
    }

    #[test]
    fn is_between_dates_higher_day() {
        let test_dates_multi_year = DateSpan {
            start_date: Date { day: 1, month: 1 },
            end_date: Date { day: 15, month: 7 },
        };

        assert!(test_dates_multi_year.is_between_dates(&Date { day: 20, month: 2 }));
    }
}
