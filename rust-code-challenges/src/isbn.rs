use std::str::FromStr;

static ISBN_LENGTH: usize = 13;
static ISBN_WEIGHTS: &'static [u8] = &[1,3];

#[derive(Debug, PartialEq)]
struct Isbn {
    raw: String,
    digits: Vec<u8>
}

#[derive(Debug, PartialEq)]
enum IsbnParseError {
    InputTooShort,
    InputTooLong,
    InvalidInputFormat,
    ChecksumFailed
}

impl FromStr for Isbn {
    type Err = IsbnParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numeric_isbn = s.replace("-", "");
        if !numeric_isbn.chars().all(char::is_numeric) {
            return Err(IsbnParseError::InvalidInputFormat);
        }
        if numeric_isbn.len() < ISBN_LENGTH {
            return Err(IsbnParseError::InputTooShort);
        }
        if numeric_isbn.len() > ISBN_LENGTH {
            return Err(IsbnParseError::InputTooLong);
        }

        let all_digits: Vec<u8> = numeric_isbn.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let input_check = *all_digits.last().unwrap();
        let digits_sum: u8 = all_digits.iter().take(all_digits.len() - 1).enumerate().map(|(i, n)| n * ISBN_WEIGHTS[i % 2]).sum();

        let calculated_check = (10 - digits_sum % 10) % 10;
        if calculated_check != input_check {
            return Err(IsbnParseError::ChecksumFailed);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: all_digits
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_input() {
        assert_eq!(Err(IsbnParseError::InvalidInputFormat), Isbn::from_str("abcd"));
    }

    #[test]
    fn short_input() {
        assert_eq!(Err(IsbnParseError::InputTooShort), Isbn::from_str("978-1-56619"));
    }

    #[test]
    fn long_input() {
        assert_eq!(Err(IsbnParseError::InputTooLong), Isbn::from_str("978-1-56619-909-4-123456"));
    }

    #[test]
    fn invalid_checksum() {
        assert_eq!(Err(IsbnParseError::ChecksumFailed), Isbn::from_str("978-1-56619-909-1"));
    }

    #[test]
    fn valid_isbns() {
        assert!(Isbn::from_str("978-1-56619-909-4").is_ok());
        assert!(Isbn::from_str("978-1-86197-876-9").is_ok());
        assert!(Isbn::from_str("978-0-545-01022-1").is_ok());
    }
}