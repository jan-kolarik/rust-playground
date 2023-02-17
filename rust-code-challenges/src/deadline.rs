use chrono::{NaiveDate, Utc};

struct ImportantEvent {
    name: String,
    date: NaiveDate
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        Utc::now().date_naive() > self.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_event() {
        let my_event = ImportantEvent {
            name: "Example".to_owned(),
            date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()
        };

        assert!(my_event.is_passed())
    }
}