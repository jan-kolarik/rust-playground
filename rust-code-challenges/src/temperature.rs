struct Temperature {
    degress: f32,
    scale: Scale
}

#[derive(PartialEq)]
enum Scale {
    Celsius,
    Fahrenheit
}

trait TemperatureConverter {
    fn to_celsius(&self) -> f32;
    fn to_fahrenheit(&self) -> f32;
}

fn fahrenheit_to_celsius(value: f32) -> f32 {
    (value - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(value: f32) -> f32 {
    value * 9.0/5.0 + 32.0
}

impl TemperatureConverter for Temperature {
    fn to_celsius(&self) -> f32 {
        if self.scale == Scale::Celsius {
            self.degress
        } else {
            fahrenheit_to_celsius(self.degress)
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        if self.scale == Scale::Fahrenheit {
            self.degress
        } else {
            celsius_to_fahrenheit(self.degress)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Scale::*;

    #[test]
    fn celsius_celsius() {
        let temperature = Temperature {
            degress: 40.0,
            scale: Celsius
        };

        assert_eq!(40.0, temperature.to_celsius());
    }

    #[test]
    fn fahrenheit_fahrenheit() {
        let temperature = Temperature {
            degress: 55.0,
            scale: Fahrenheit
        };

        assert_eq!(55.0, temperature.to_fahrenheit());
    }

    #[test]
    fn celsius_fahrenheit() {
        let temperature = Temperature {
            degress: 25.0,
            scale: Celsius
        };

        assert_eq!(celsius_to_fahrenheit(25.0), temperature.to_fahrenheit());
    }

    #[test]
    fn fahrenheit_celsius() {
        let temperature = Temperature {
            degress: 65.5,
            scale: Fahrenheit
        };

        assert_eq!(fahrenheit_to_celsius(65.5), temperature.to_celsius());
    }
}