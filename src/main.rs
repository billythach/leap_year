use chrono::{Datelike, DateTime, TimeZone};

trait LeapYearOperation {
    fn is_leap_year(&self) -> bool;
}

impl<Tz: TimeZone> LeapYearOperation for DateTime<Tz> {
    fn is_leap_year(&self) -> bool {
        let year_is_divisible_by_400 = match self.year() % 400 {
            0 => { true }
            _ => { false }
        };

        let year_is_divisible_by_4 = match self.year() % 4 {
            0 => { true }
            _ => { false }
        };

        let year_is_divisible_by_100 = match self.year() % 100 {
            0 => { true }
            _ => { false }
        };

        return match (year_is_divisible_by_400, year_is_divisible_by_4, year_is_divisible_by_100) {
            (true, _, _) => { true }
            (_, true, false) => { true }
            // (false, _, true) => { false }
            // (t_, false, _) => { false }
            (_, _, _) => { false }
        };
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use chrono::{DateTime};

    use crate::LeapYearOperation;

    #[test]
    fn year_divisible_400_by_is_a_leap_year() {
        let date = DateTime::parse_from_rfc3339("2000-12-19T16:39:57-08:00").unwrap();
        assert!(date.is_leap_year(), "2000 is a leap year");
    }

    #[test]
    fn year_not_divisible_400_by_is_not_a_leap_year() {
        let date = DateTime::parse_from_rfc3339("2001-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "2001 is NOT a leap year");
    }

    #[test]
    fn year_divisible_100_but_not_400_by_is_not_a_leap_year() {
        let date = DateTime::parse_from_rfc3339("1700-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "1700 is NOT a leap year");
        let date = DateTime::parse_from_rfc3339("2100-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "2100 is NOT a leap year");
    }

    #[test]
    fn year_divisible_4_but_not_100_by_is_a_leap_year() {
        let date = DateTime::parse_from_rfc3339("2008-12-19T16:39:57-08:00").unwrap();
        assert!(date.is_leap_year(), "2008 is a leap year");
        let date = DateTime::parse_from_rfc3339("2012-12-19T16:39:57-08:00").unwrap();
        assert!(date.is_leap_year(), "2012 is a leap year");
        let date = DateTime::parse_from_rfc3339("2016-12-19T16:39:57-08:00").unwrap();
        assert!(date.is_leap_year(), "2016 is a leap year");
    }

    #[test]
    fn year_not_divisible_4_but_not_100_by_is_not_a_leap_year() {
        let date = DateTime::parse_from_rfc3339("2017-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "2017 is NOT a leap year");
        let date = DateTime::parse_from_rfc3339("2018-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "2018 is NOT a leap year");
        let date = DateTime::parse_from_rfc3339("2019-12-19T16:39:57-08:00").unwrap();
        assert!(!date.is_leap_year(), "2019 is NOT a leap year");
    }

}