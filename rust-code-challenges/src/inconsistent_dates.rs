use chrono::NaiveDate;

fn parse_date(text: &str) -> Option<NaiveDate> {
    let allowed_formats = [
        "%Y %b %d",
        "%Y-%m-%d",
        "%Y/%B/%d",
        "%d.%b.%Y",
        "%b.%d.%Y"
    ];

    for format in allowed_formats {
        if let Ok(date) = NaiveDate::parse_from_str(text, format) {
            return Some(date);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_test_1() {
        assert_eq!(parse_date("2002 Feb 02"), NaiveDate::from_ymd_opt(2002, 2, 2));
    }

    #[test]
    fn date_test_2() {
        assert_eq!(parse_date("2010-12-11"), NaiveDate::from_ymd_opt(2010, 12, 11));
    }

    #[test]
    fn date_test_3() {
        assert_eq!(parse_date("1999/March/02"), NaiveDate::from_ymd_opt(1999, 3, 2));
    }

    #[test]
    fn date_test_4() {
        assert_eq!(parse_date("01.Mar.2021"), NaiveDate::from_ymd_opt(2021, 3, 1));
    }

    #[test]
    fn date_test_5() {
        assert_eq!(parse_date("Mar.05.2021"), NaiveDate::from_ymd_opt(2021, 3, 5));
    }

    #[test]
    fn invalid_dates_test() {
        assert_eq!(parse_date("xyz"), None);
        assert_eq!(parse_date("Mer.05.2021"), None);
        assert_eq!(parse_date("Jan.Feb.2021"), None);
        assert_eq!(parse_date("1000-01-32"), None);
        assert_eq!(parse_date("1999/June/00"), None);
    }
}