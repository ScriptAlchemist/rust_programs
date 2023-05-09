/// TempType is the temperature in Celcius or Fahreheit
#[derive(Debug)]
enum TempType {
    Celcius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

/// Thermometer structure before modifying methods
#[derive(Debug)]
struct Temperature {
    temp_t: TempType,
}

/// Thermometer implementation
impl Thermometer {

    fn new(temp_type: TempType) -> Thermometer {
        Thermometer {
            temp_t: temp_type,
        }
    }

    fn switch_to_celcius(&self) -> Thermometer {
        match self.temp_t {
            TempType::Celcius(temp) => {
                Thermometer {
                    temp_t: TempType::Fahrenheit(c_to_f(temp).round()),
                }
            }
            TempType::Fahrenheit(temp) => {
                Thermometer {
                    temp_t: TempType::Celcius(f_to_c(temp).round()),
                }
            }
            TempType::Kelvin(temp) => {
                Thermometer {
                    temp_t: ,
                }
            }
        }

    }

    fn switch_to_fahrenheit(&self) -> Thermometer {
        match self.temp_t {
            TempType::Celcius(temp) => {
                Thermometer {
                    temp_t: TempType::Fahrenheit(c_to_f(temp).round()),
                }
            }
            TempType::Fahrenheit(temp) => {
                Thermometer {
                    temp_t: TempType::Celcius(f_to_c(temp).round()),
                }
            }
            TempType::Kelvin(temp) => {
                Thermometer {
                    temp_t: ,
                }
            }
        }
    }

}

fn c_to_f(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}

fn c_to_k(temp: f64) -> f64 {

}

fn k_to_c(temp: f64) -> f64 {

}

fn f_to_k(temp: f64) -> f64 {

}

fn k_to_f(temp: f64) -> f64 {

}

// C = (F - 32) * 5/9
fn main() {
    let mut test_celcius = Thermometer::new(TempType::Celcius(20_f64));
    let mut test_fahreheit= Thermometer::new(TempType::Fahrenheit(89_f64));

    println!("Celcius temp: {:?}", test_celcius.temp_t);
    println!("Fahreheit temp: {:?}", test_fahreheit.temp_t);

    // Switch Temp Types
    test_celcius = test_celcius.switch_to_fahrenheit();
    test_fahreheit = test_fahreheit.switch_to_celcius();
    println!("Celcius temp: {:?}", test_celcius.temp_t);
    println!("Fahreheit temp: {:?}", test_fahreheit.temp_t);
}
