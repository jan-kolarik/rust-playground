use std::str::FromStr;

static RGB_STRING_LENGTH: usize = 7;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

#[derive(Debug, PartialEq)]
enum RgbParseError {
    IncorrectInputLength,
    MissingHashChar,
    InvalidRedFormat,
    InvalidGreenFormat,
    InvalidBlueFormat
}

impl FromStr for Rgb {
    type Err = RgbParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != RGB_STRING_LENGTH {
            return Err(RgbParseError::IncorrectInputLength);
        }

        let hex_option = s.strip_prefix('#');
        if hex_option == None {
            return Err(RgbParseError::MissingHashChar);
        }

        let hex = hex_option.unwrap();
        Ok(Rgb {
            r: u8::from_str_radix(&hex[0..2], 16).or_else(|_| Err(RgbParseError::InvalidRedFormat))?,
            g: u8::from_str_radix(&hex[2..4], 16).or_else(|_| Err(RgbParseError::InvalidGreenFormat))?,
            b: u8::from_str_radix(&hex[4..6], 16).or_else(|_| Err(RgbParseError::InvalidBlueFormat))?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_length() {
        assert_eq!(Err(RgbParseError::IncorrectInputLength), Rgb::from_str("123456789"));
        assert_eq!(Err(RgbParseError::IncorrectInputLength), Rgb::from_str("#ABCDEFGH"));
    }

    #[test]
    fn missing_hash_char() {
        assert_eq!(Err(RgbParseError::MissingHashChar), Rgb::from_str("ABCDEFG"));
        assert_eq!(Err(RgbParseError::MissingHashChar), Rgb::from_str("1234567"));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(Err(RgbParseError::InvalidRedFormat), Rgb::from_str("#GHCD12"));
        assert_eq!(Err(RgbParseError::InvalidGreenFormat), Rgb::from_str("#Aa%^&*"));
        assert_eq!(Err(RgbParseError::InvalidBlueFormat), Rgb::from_str("#89ff  "));
    }

    #[test]
    fn valid_rgbs() {
        assert!(Rgb::from_str("#dadada").is_ok());
        assert!(Rgb::from_str("#DADADA").is_ok());
        assert!(Rgb::from_str("#ededed").is_ok());
        assert!(Rgb::from_str("#000000").is_ok());
        assert!(Rgb::from_str("#a9B193").is_ok());
    }

    #[test]
    fn decimal_conversion() {
        let rgb_same_channels = Rgb::from_str("#aaaaaa").unwrap();
        assert_eq!(170, rgb_same_channels.r());
        assert_eq!(170, rgb_same_channels.g());
        assert_eq!(170, rgb_same_channels.b());

        let rgb_different_channels = Rgb::from_str("#00cd34").unwrap();
        assert_eq!(0, rgb_different_channels.r());
        assert_eq!(205, rgb_different_channels.g());
        assert_eq!(52, rgb_different_channels.b());
    }
}