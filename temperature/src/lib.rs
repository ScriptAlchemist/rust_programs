#[allow(dead_code)]

/// TempType is the temperature type with a f64 value for the temperature it includes
/// Celcius, Fahrenheit and Kelvin
#[derive(Debug)]
pub enum TempType {
    Celcius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

/// Temperature structure with internal temp t.
#[derive(Debug)]
pub struct Temperature {
    pub t: TempType,
}

/// Temperature implementation
impl Temperature {

    pub fn new(temp_type: TempType) -> Temperature {
        Temperature {
            t: temp_type,
        }
    }

    pub fn switch_to_celcius(&self) -> Temperature {
        match self.t {
            TempType::Celcius(temp) => {
                Temperature::new(TempType::Celcius(temp))
            }
            TempType::Fahrenheit(temp) => {
                Temperature::new(TempType::Celcius(fahrenheit_to_celcius(temp)))
            }
            TempType::Kelvin(temp) => {
                Temperature::new(TempType::Celcius(kelvin_to_celcius(temp)))
            }
        }
    }

    pub fn switch_to_fahrenheit(&self) -> Temperature {
        match self.t {
            TempType::Celcius(temp) => {
                Temperature::new(TempType::Fahrenheit(celcius_to_fahrenheit(temp)))
            }
            TempType::Fahrenheit(temp) => {
                Temperature::new(TempType::Fahrenheit(temp))
            }
            TempType::Kelvin(temp) => {
                Temperature::new(TempType::Fahrenheit(kelvin_to_fahrenheit(temp)))
            }
        }
    }

    pub fn switch_to_kelvin(&self) -> Temperature {
        match self.t {
            TempType::Celcius(temp) => {
                Temperature::new(TempType::Kelvin(celcius_to_kelvin(temp)))
            }
            TempType::Fahrenheit(temp) => {
                Temperature::new(TempType::Kelvin(fahrenheit_to_kelvin(temp)))
            }
            TempType::Kelvin(temp) => {
                Temperature::new(TempType::Kelvin(temp))
            }
        }
    }
}

// Private Conversions

/// Celcius to Kelvin conversion calculation
fn celcius_to_kelvin(temp: f64) -> f64 {
    temp + 273.15
}

/// Kelvin to Celcius conversion calculation
fn kelvin_to_celcius(temp: f64) -> f64 {
    temp - 273.15
}

/// Fahrenheit to Kelvin conversion calculation
fn fahrenheit_to_kelvin(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0 + 273.15
}

/// Kelvin to Fahrenheit conversion calculation
fn kelvin_to_fahrenheit(temp: f64) -> f64 {
    (temp - 273.15) * 9.0/5.0 + 32.0
}

/// Celcius to Fahrenheit conversion calculation
fn celcius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

/// Fahrenheit to Celcius conversion calculation
fn fahrenheit_to_celcius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

// Private Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celcius_to_kelvin() {
        assert_eq!(fahrenheit_to_kelvin(32.0), 273.15);
        assert_eq!(fahrenheit_to_kelvin(97.0), 309.26111111111106);
    }

    #[test]
    fn test_kelvin_to_celcius () {
        assert_eq!(kelvin_to_celcius(273.15), 0.0);
        assert_eq!(kelvin_to_celcius(0.2453234532), -272.9046765468);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert_eq!(fahrenheit_to_kelvin(87.0), 303.7055555555555);
        assert_eq!(fahrenheit_to_kelvin(-140.0), 177.59444444444443);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert_eq!(kelvin_to_fahrenheit(34.0), -398.46999999999997);
        assert_eq!(kelvin_to_fahrenheit(0.8923434), -458.0637818799999);
    }

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert_eq!(celcius_to_fahrenheit(34.0), 93.2);
        assert_eq!(celcius_to_fahrenheit(93.0), 199.4);
    }

    #[test]
    fn test_fahrenheit_to_celcius() {
        assert_eq!(fahrenheit_to_celcius(34.0), 1.1111111111111112);
        assert_eq!(fahrenheit_to_celcius(92.0), 33.333333333333336);
    }
}

