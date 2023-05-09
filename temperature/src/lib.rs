/// TempType is the temperature type with a f64 value for the temperature it includes
/// Celsius, Fahrenheit and Kelvin
#[derive(Debug)]
pub enum TempType {
    Celsius(f64),
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

    /// Create a new temp structure with an internal temperature inside
    ///
    /// # Examples
    ///
    /// ```
    /// use temps::{Temperature, TempType};
    /// let celsius_temp = Temperature::new(TempType::Celsius(43_f64));
    /// let fahrenheit_temp = Temperature::new(TempType::Fahrenheit(43_f64));
    /// let kelvin_temp = Temperature::new(TempType::Kelvin(43_f64));
    ///
    /// match celsius_temp.t {
    ///   TempType::Celsius(temp) => assert_eq!(temp, 43_f64),
    ///   _ => panic!("Should be Celsius"),
    /// }
    ///
    /// match fahrenheit_temp.t {
    ///   TempType::Fahrenheit(temp) => assert_eq!(temp, 43_f64),
    ///   _ => panic!("Should be Fahrenheit"),
    /// }
    ///
    /// match kelvin_temp.t {
    ///   TempType::Kelvin(temp) => assert_eq!(temp, 43_f64),
    ///   _ => panic!("Should be Kelvin"),
    /// }
    ///
    /// ```
    pub fn new(temp_type: TempType) -> Self {
        Temperature {
            t: temp_type,
        }
    }

    /// Convert your existing Temperature into Celsius
    ///
    /// # Examples
    ///
    /// ```
    /// use temps::{Temperature, TempType};
    /// let celsius_temp = Temperature::new(TempType::Celsius(43_f64));
    /// let fahrenheit_temp = Temperature::new(TempType::Fahrenheit(43_f64));
    /// let kelvin_temp = Temperature::new(TempType::Kelvin(43_f64));
    ///
    /// let celsius_to_celsius = celsius_temp.switch_to_celsius();
    /// let fahrenheit_to_celsius = fahrenheit_temp.switch_to_celsius();
    /// let kelvin_to_celsius = kelvin_temp.switch_to_celsius();
    ///
    /// // Check celsius switch to celsius
    /// match (celsius_temp.t, celsius_to_celsius.t) {
    ///   (TempType::Celsius(val1), TempType::Celsius(val2)) => {
    ///     assert_eq!(val1 == val2, true)
    ///   }
    ///   _ => panic!("Needs to be Celsius TempType"),
    /// };
    ///
    /// // Check fahrenheit switch to celsius
    /// match fahrenheit_to_celsius.t {
    ///   TempType::Celsius(temp) => assert_eq!(temp, 6.111111111111111),
    ///   _ => panic!("Needs to be Celsius TempType"),
    /// };
    ///
    /// // Check kelvin switch to celsius
    /// match kelvin_to_celsius.t {
    ///   TempType::Celsius(temp) => assert_eq!(temp, -230.14999999999998),
    ///   _ => panic!("Needs to be Celsius TempType"),
    /// };
    ///
    /// ```
    pub fn switch_to_celsius(&self) -> Self {
        match self.t {
            TempType::Celsius(temp) => {
                Temperature::new(TempType::Celsius(temp))
            }
            TempType::Fahrenheit(temp) => {
                Temperature::new(TempType::Celsius(fahrenheit_to_celsius(temp)))
            }
            TempType::Kelvin(temp) => {
                Temperature::new(TempType::Celsius(kelvin_to_celsius(temp)))
            }
        }
    }

    /// Convert your existing Temperature into Fahrenheit
    ///
    /// # Examples
    ///
    /// ```
    /// use temps::{Temperature, TempType};
    /// let celsius_temp = Temperature::new(TempType::Celsius(43_f64));
    /// let fahrenheit_temp = Temperature::new(TempType::Fahrenheit(43_f64));
    /// let kelvin_temp = Temperature::new(TempType::Kelvin(43_f64));
    ///
    /// let celsius_to_fahrenheit = celsius_temp.switch_to_fahrenheit();
    /// let fahrenheit_to_fahrenheit = fahrenheit_temp.switch_to_fahrenheit();
    /// let kelvin_to_fahrenheit = kelvin_temp.switch_to_fahrenheit();
    ///
    /// // Check celsius switch to fahrenheit
    /// match celsius_to_fahrenheit.t {
    ///   TempType::Fahrenheit(temp) => assert_eq!(temp, 109.4),
    ///   _ => panic!("Needs to be Fahrenheit TempType"),
    /// };
    ///
    /// // Check fahrenheit switch to fahrenheit
    /// match (fahrenheit_temp.t, fahrenheit_to_fahrenheit.t) {
    ///   (TempType::Fahrenheit(val1), TempType::Fahrenheit(val2)) => {
    ///     assert_eq!(val1 == val2, true)
    ///   }
    ///   _ => panic!("Needs to be Fahrenheit TempType"),
    /// };
    ///
    /// // Check kelvin switch to fahrenheit
    /// match kelvin_to_fahrenheit.t {
    ///   TempType::Fahrenheit(temp) => assert_eq!(temp, -382.27),
    ///   _ => panic!("Needs to be Fahrenheit TempType"),
    /// };
    ///
    /// ```
    pub fn switch_to_fahrenheit(&self) -> Self {
        match self.t {
            TempType::Celsius(temp) => {
                Temperature::new(TempType::Fahrenheit(celsius_to_fahrenheit(temp)))
            }
            TempType::Fahrenheit(temp) => {
                Temperature::new(TempType::Fahrenheit(temp))
            }
            TempType::Kelvin(temp) => {
                Temperature::new(TempType::Fahrenheit(kelvin_to_fahrenheit(temp)))
            }
        }
    }


    /// Convert your existing Temperature into Kelvin
    ///
    /// # Examples
    ///
    /// ```
    /// use temps::{Temperature, TempType};
    /// let celsius_temp = Temperature::new(TempType::Celsius(43_f64));
    /// let fahrenheit_temp = Temperature::new(TempType::Fahrenheit(43_f64));
    /// let kelvin_temp = Temperature::new(TempType::Kelvin(43_f64));
    ///
    /// let celsius_to_kelvin = celsius_temp.switch_to_kelvin();
    /// let fahrenheit_to_kelvin = fahrenheit_temp.switch_to_kelvin();
    /// let kelvin_to_kelvin = kelvin_temp.switch_to_kelvin();
    ///
    /// // Check celsius switch to kelvin
    /// match celsius_to_kelvin.t {
    ///   TempType::Kelvin(temp) => assert_eq!(temp, 316.15),
    ///   _ => panic!("Needs to be Kelvin TempType"),
    /// };
    ///
    /// // Check fahrenheit switch to kelvin
    /// match fahrenheit_to_kelvin.t {
    ///   TempType::Kelvin(temp) => assert_eq!(temp, 279.26111111111106),
    ///   _ => panic!("Needs to be Kelvin Temptype"),
    /// };
    ///
    /// ```
    ///
    pub fn switch_to_kelvin(&self) -> Self {
        match self.t {
            TempType::Celsius(temp) => {
                Temperature::new(TempType::Kelvin(celsius_to_kelvin(temp)))
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

/// Celsius to Kelvin conversion calculation
fn celsius_to_kelvin(temp: f64) -> f64 {
    temp + 273.15
}

/// Kelvin to Celsius conversion calculation
fn kelvin_to_celsius(temp: f64) -> f64 {
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

/// Celsius to Fahrenheit conversion calculation
fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

/// Fahrenheit to Celsius conversion calculation
fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

// Private Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(fahrenheit_to_kelvin(32.0), 273.15);
        assert_eq!(fahrenheit_to_kelvin(97.0), 309.26111111111106);
    }

    #[test]
    fn test_kelvin_to_celsius () {
        assert_eq!(kelvin_to_celsius(273.15), 0.0);
        assert_eq!(kelvin_to_celsius(0.2453234532), -272.9046765468);
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
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(34.0), 93.2);
        assert_eq!(celsius_to_fahrenheit(93.0), 199.4);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(34.0), 1.1111111111111112);
        assert_eq!(fahrenheit_to_celsius(92.0), 33.333333333333336);
    }
}

