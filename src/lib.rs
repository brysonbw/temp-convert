//! > **Command line tool to convert between common temperature units**
//!
//! ## Install
//! ```console
//! $ cargo install temp-convert
//! ```
//!
//! ## Example
//!```console
//! $ temp-convert 7 -u c -c k
//! ```
//!
//! ## Usage
//! ```console
//! temp-convert OPTIONS VALUE
//! ```
//!
//! Arguments:
//!
//! VALUE, Temperature value to convert
//!
//! Options:
//!   
//!   -u, --unit
//!           Temperature unit of the provided value (Celsius, Fahrenheit, or Kelvin)
//!
//!   -c, --convert
//!           Target temperature unit to convert the value to (Celsius, Fahrenheit, or Kelvin)
//!
//!   -h, --help
//!           Print help (see a summary with '-h')
//!
//!   -V, --version
//!           Print version
//!

/// Constant/helpers
pub mod utils;

use std::error::Error;

use crate::utils::{
    ABS_ZERO_CELSIUS, ABS_ZERO_FAHRENHEIT, ABS_ZERO_KELVIN, COLOR_GREEN, COLOR_RESET,
};
use clap::Parser;

/// Tempeature unit
#[derive(clap::ValueEnum, Clone, Debug)]
enum Unit {
    #[value(alias = "celsius")]
    C,

    #[value(alias = "fahrenheit")]
    F,

    #[value(alias = "kelvin")]
    K,
}

impl Unit {
    /// Unit absolute zero value
    fn absolute_zero(&self) -> f64 {
        match self {
            Unit::C => ABS_ZERO_CELSIUS,
            Unit::F => ABS_ZERO_FAHRENHEIT,
            Unit::K => ABS_ZERO_KELVIN,
        }
    }

    fn full_name(&self) -> &str {
        match self {
            Unit::C => "Celsius",
            Unit::F => "Fahrenheit",
            Unit::K => "Kelvin",
        }
    }

    /// Convert a temperature value from the current unit to Celsius
    fn to_celsius(&self, value: f64) -> f64 {
        match self {
            Unit::C => value,
            Unit::F => (value - 32.0) * 5.0 / 9.0,
            Unit::K => value - 273.15,
        }
    }

    /// Convert a temperature value from Celsius to the current unit
    fn from_celsius(&self, celsius: f64) -> f64 {
        match self {
            Unit::C => celsius,
            Unit::F => (celsius * 9.0 / 5.0) + 32.0,
            Unit::K => celsius + 273.15,
        }
    }
}

/// Converts temperature values between Celsius, Fahrenheit, and Kelvin
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Convert temperatures between Celsius, Fahrenheit, and Kelvin.",
    long_about = "Converts temperature values between Celsius, Fahrenheit, and Kelvin."
)]
pub struct Args {
    /// Temperature value to convert
    #[arg(allow_hyphen_values = true)]
    value: f64,

    /// Temperature unit of the provided value (Celsius, Fahrenheit, or Kelvin)
    #[arg(short = 'u', long = "unit", ignore_case = true, default_value = "f")]
    value_unit: Unit,

    /// Target temperature unit to convert the value to (Celsius, Fahrenheit, or Kelvin)
    #[arg(
        short = 'c',
        long = "convert",
        value_enum,
        ignore_case = true,
        default_value = "c"
    )]
    convert: Unit,
}

impl Args {
    /// Run/execute command line arguments
    pub fn run(self) -> Result<String, Box<dyn Error>> {
        // Validate value
        let min: f64 = self.value_unit.absolute_zero();
        if self.value < min {
            return Err(format!(
                "Value {} is below absolute zero for {} ({})",
                self.value,
                self.value_unit.full_name(),
                min
            )
            .into());
        }

        // Convert value
        let result: f64 = self
            .convert
            .from_celsius(self.value_unit.to_celsius(self.value));

        Ok(format!(
            "{}{:.2}°{} is {:.2}°{}{}",
            COLOR_GREEN,
            self.value,
            self.value_unit.full_name(),
            result,
            self.convert.full_name(),
            COLOR_RESET
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Constants and helpers
    const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

    /// Small constant to handle floating-point precision issues
    const EPSILON: f64 = 1e-10;
    fn assert_approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPSILON,
            "Assertion failed: {} is not approximately {}",
            a,
            b
        );
    }

    /// Check if the output/result string contains the expected substrings.
    /// Ignores color code constant/strings
    fn contains_all(output: &str, sub_strings: &[&str]) -> bool {
        sub_strings.iter().all(|&n| output.contains(n))
    }

    #[test]
    fn test_absolute_zero_values() {
        assert_eq!(Unit::C.absolute_zero(), ABS_ZERO_CELSIUS);
        assert_eq!(Unit::F.absolute_zero(), ABS_ZERO_FAHRENHEIT);
        assert_eq!(Unit::K.absolute_zero(), ABS_ZERO_KELVIN);
    }

    #[test]
    fn test_full_names() {
        assert_eq!(Unit::C.full_name(), "Celsius");
        assert_eq!(Unit::F.full_name(), "Fahrenheit");
        assert_eq!(Unit::K.full_name(), "Kelvin");
    }

    #[test]
    fn test_to_celsius() {
        // From Fahrenheit
        assert_approx_eq(Unit::F.to_celsius(32.0), 0.0);
        assert_approx_eq(Unit::F.to_celsius(212.0), 100.0);
        assert_approx_eq(Unit::F.to_celsius(-40.0), -40.0);

        // From Kelvin
        assert_approx_eq(Unit::K.to_celsius(273.15), 0.0);
        assert_approx_eq(Unit::K.to_celsius(0.0), -273.15);

        // From Celsius
        assert_approx_eq(Unit::C.to_celsius(25.0), 25.0);
    }

    #[test]
    fn test_from_celsius() {
        // To Fahrenheit
        assert_approx_eq(Unit::F.from_celsius(0.0), 32.0);
        assert_approx_eq(Unit::F.from_celsius(100.0), 212.0);
        assert_approx_eq(Unit::F.from_celsius(-40.0), -40.0);

        // To Kelvin
        assert_approx_eq(Unit::K.from_celsius(0.0), 273.15);
        assert_approx_eq(Unit::K.from_celsius(-273.15), 0.0);

        // To Celsius
        assert_approx_eq(Unit::C.from_celsius(36.6), 36.6);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_temp: f64 = 98.6; // Body temp in Fahrenheit
        let celsius: f64 = Unit::F.to_celsius(original_temp);
        let back_to_f: f64 = Unit::F.from_celsius(celsius);

        assert_approx_eq(original_temp, back_to_f);
    }

    // CLI/Args
    #[test]
    fn test_valid_conversion_f_to_c() {
        let args: Args = Args {
            value: 32.0,
            value_unit: Unit::F,
            convert: Unit::C,
        };

        let output: String = args.run().expect("Failed conversion");
        assert!(contains_all(
            &output,
            &["32.00", Unit::F.full_name(), "0.00", Unit::C.full_name()]
        ));
    }

    #[test]
    fn test_valid_conversion_c_to_k() {
        let args: Args = Args {
            value: 0.0,
            value_unit: Unit::C,
            convert: Unit::K,
        };

        let output: String = args.run().expect("Failed conversion");
        assert!(contains_all(
            &output,
            &["0.00", Unit::C.full_name(), "273.15", Unit::K.full_name()]
        ));
    }

    #[test]
    fn test_absolute_zero_c_error() {
        let args: Args = Args {
            value: ABS_ZERO_CELSIUS - 1.0,
            value_unit: Unit::C,
            convert: Unit::F,
        };

        let output: Result<String, Box<dyn Error>> = args.run();
        assert!(output.is_err());
        let error_msg: String = output.unwrap_err().to_string();
        assert!(error_msg.contains("below absolute zero"));
        assert!(error_msg.contains(Unit::C.full_name()));
        assert!(error_msg.contains(&ABS_ZERO_CELSIUS.to_string()));
    }

    #[test]
    fn test_absolute_zero_f_error() {
        let args: Args = Args {
            value: ABS_ZERO_FAHRENHEIT - 1.0,
            value_unit: Unit::F,
            convert: Unit::C,
        };

        let output: Result<String, Box<dyn Error>> = args.run();
        assert!(output.is_err());
        let error_msg: String = output.unwrap_err().to_string();
        assert!(error_msg.contains("below absolute zero"));
        assert!(error_msg.contains(Unit::F.full_name()));
        assert!(error_msg.contains(&ABS_ZERO_FAHRENHEIT.to_string()));
    }

    #[test]
    fn test_absolute_zero_k_error() {
        let args: Args = Args {
            value: ABS_ZERO_KELVIN - 1.0,
            value_unit: Unit::K,
            convert: Unit::C,
        };

        let output: Result<String, Box<dyn Error>> = args.run();
        assert!(output.is_err());
        let error_msg: String = output.unwrap_err().to_string();
        assert!(error_msg.contains("below absolute zero"));
        assert!(error_msg.contains(Unit::K.full_name()));
        assert!(error_msg.contains(&ABS_ZERO_KELVIN.to_string()));
    }

    #[test]
    fn test_negative_c_allowed() {
        let args: Args = Args {
            value: -40.0,
            value_unit: Unit::C,
            convert: Unit::F,
        };

        let output: String = args
            .run()
            .expect("Should allow negative Celsius above absolute zero");
        assert!(output.contains("-40.00"));
    }

    #[test]
    fn test_negative_f_allowed() {
        let args: Args = Args {
            value: -40.0,
            value_unit: Unit::F,
            convert: Unit::C,
        };

        let output: String = args
            .run()
            .expect("Should allow negative Fahrenheit above absolute zero");
        assert!(output.contains("-40.00"));
    }

    #[test]
    fn test_conversion_crossover_point() {
        // -40 Celsius is -40 Fahrenheit
        let args = Args {
            value: -40.0,
            value_unit: Unit::C,
            convert: Unit::F,
        };

        let output: String = args.run().expect("Failed conversion");
        assert!(output.contains("-40.00"));
    }

    #[test]
    fn test_parsing_defaults() {
        let args: Args = Args::parse_from([PACKAGE_NAME, "100"]);
        assert_eq!(args.value, 100.0);
        assert!(matches!(args.value_unit, Unit::F));
        assert!(matches!(args.convert, Unit::C));
    }
}
