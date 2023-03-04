use chrono::NaiveDate;

fn parse_naive_date_from_str(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

fn weeks_between(a: &str, b: &str) -> i64 {
    let first_date = parse_naive_date_from_str(a);
    let second_date = parse_naive_date_from_str(b);
    (second_date - first_date).num_weeks()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_week() {
        let first_date = "2020-01-02";
        let second_date = "2020-01-05";
        assert_eq!(0, weeks_between(first_date, second_date));
    }

    #[test]
    fn week_after() {
        let first_date = "2000-06-01";
        let second_date = "2000-06-10";
        assert_eq!(1, weeks_between(first_date, second_date));
    }

    #[test]
    fn week_before() {
        let first_date = "2000-06-10";
        let second_date = "2000-06-01";
        assert_eq!(-1, weeks_between(first_date, second_date));
    }

    #[test]
    fn several_weeks_between() {
        let first_date = "2022-06-01";
        let second_date = "2022-06-22";
        assert_eq!(3, weeks_between(first_date, second_date));
    }
}