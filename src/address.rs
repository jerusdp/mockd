//! Generate mock date addresses
//!
//! # Example
//!
//!
//! ```rust
//! use mockd::address;
//!
//! fn main() {
//!     let data = address::info(); // address::Info struct
//!     let data = address::street(); // street: 1128 South North Dakota borough
//!     let data = address::street_number(); // street_number: 3155
//!     let data = address::street_prefix(); // street_prefix: Port
//!     let data = address::street_name(); // street_name: Kansas
//!     let data = address::street_suffix(); // street_suffix: mouth
//!     let data = address::city(); // city: Schmelerburgh
//!     let data = address::state(); // state: Kentucky
//!     let data = address::state_abr(); // state_abr: WA
//!     let data = address::zip(); // zip: 75221
//!     let data = address::country(); // country: Romania
//!     let data = address::country_abr(); // country_abr: BI
//!     let data = address::latitude(); // latitude: -69.14192
//!     let data = address::latitude_in_range(-30 as f64, 30 as f64); // latitude_in_range: -18.35571     let data = address::longitude(); // longitude: 113.12952
//!     let data = address::longitude_in_range(-30 as f64, 30 as f64); // longitude_in_range: -16.484156
//! }
//! ```

pub(crate) use crate::data::address;
use crate::misc;
use crate::name;
// use ::std::string::String;

/// The elements of an address including
///  address:   combines the street, city, state and zip components
///  street:    the street name
///  city:      the city name
///  state:     the state name
///  zip:       the postal code for the address
///  country:   the country name
///  latitude:  the latitude of the address
///  longitude: the longitude of the address
///

#[allow(dead_code)]
pub struct Info {
    address: String,
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

/// Create a new HcaptchaRequest from only the response string
///
/// # Input
///
/// The Hcaptcha API has two mandatory parameters:
///     secret:     The client's secret key for authentication
///     response:    The response code to validate
///
/// # Output
///
/// HcaptchaRequest is returned if the inputs are valid.
/// [HcaptchaError] is returned if the validation fails.
///
/// # Example
///
/// ``` no_run
///     use hcaptcha::HcaptchaRequest;
/// # fn main() -> Result<(), hcaptcha::HcaptchaError>{
///     let secret = get_your_secret();     // your secret key
///     let response = get_response();    // Hcaptcha client response
///
///     let request = HcaptchaRequest::new_from_response(&secret, &response)?;
/// # Ok(())
/// # }
/// # fn get_your_secret() -> String {
/// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
/// # }
/// # use rand::distributions::Alphanumeric;
/// # use rand::{thread_rng, Rng};
/// # use std::iter;
/// # fn get_response() -> String {
/// #    let mut rng = thread_rng();
/// #    iter::repeat(())
/// #        .map(|()| rng.sample(Alphanumeric))
/// #        .map(char::from)
/// #        .take(100)
/// #        .collect()
/// # }
///  ```
/// # Logging
///
/// If the tracing feature is enabled a debug level span is set for the
/// method.
/// The secret field will not be logged.
///

pub fn info() -> Info {
    Info {
        address: format!("{}, {}, {} {}", street(), city(), state(), zip()),
        street: street(),
        city: city(),
        state: state(),
        zip: zip(),
        country: country(),
        latitude: latitude(),
        longitude: longitude(),
    }
}

pub fn street() -> String {
    match misc::random::<i64>(1, 2) {
        1 => {
            return format!(
                "{} {} {} {}",
                street_number(),
                street_prefix(),
                street_name(),
                street_suffix()
            )
        }
        2 => return format!("{} {} {}", street_number(), street_name(), street_suffix()),
        _ => "impossible".to_string(),
    }
}

pub fn street_number() -> String {
    misc::replace_with_numbers(misc::random_data(address::NUMBER).to_string())
}

pub fn street_prefix() -> String {
    misc::random_data(address::STREET_PREFIX).to_string()
}

pub fn street_name() -> String {
    misc::random_data(address::STATE).to_string()
}

pub fn street_suffix() -> String {
    misc::random_data(address::STREET_SUFFIX).to_string()
}

pub fn city() -> String {
    match misc::random::<i64>(1, 3) {
        1 => return format!("{}{}", name::first(), street_suffix()),
        2 => return format!("{}{}", name::last(), street_suffix()),
        3 => return format!("{} {}", street_prefix(), name::last()),
        _ => "impossible".to_string(),
    }
}

pub fn state() -> String {
    misc::random_data(address::STATE).to_string()
}

pub fn state_abr() -> String {
    misc::random_data(address::STATE_ABR).to_string()
}

pub fn zip() -> String {
    misc::replace_with_numbers(misc::random_data(address::ZIP).to_string())
}

pub fn country() -> String {
    misc::random_data(address::COUNTRY).to_string()
}

pub fn country_abr() -> String {
    misc::random_data(address::COUNTRY_ABR).to_string()
}

pub fn latitude() -> f32 {
    misc::random::<f32>(-90.0, 90.0)
}

pub fn latitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || min < -90.0 || min > 90.0 || max < -90.0 || max > 90.0 {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

pub fn longitude() -> f32 {
    misc::random::<f32>(-180.0, 180.0)
}

pub fn longitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || min < -180.0 || min > 180.0 || max < -180.0 || max > 180.0 {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

#[cfg(test)]
mod tests {
    use crate::address;
    use crate::testify::exec_mes;

    #[test]
    fn street() {
        exec_mes("address::street", address::street);
    }

    #[test]
    fn street_number() {
        exec_mes("address::street_number", address::street_number);
    }

    #[test]
    fn street_prefix() {
        exec_mes("address::street_prefix", address::street_prefix);
    }

    #[test]
    fn street_name() {
        exec_mes("address::street_name", address::street_name);
    }

    #[test]
    fn street_suffix() {
        exec_mes("address::street_suffix", address::street_suffix);
    }

    #[test]
    fn city() {
        exec_mes("address::city", address::city);
    }

    #[test]
    fn state() {
        exec_mes("address::state", address::state);
    }

    #[test]
    fn state_abr() {
        exec_mes("address::state_abr", address::state_abr);
    }

    #[test]
    fn zip() {
        exec_mes("address::zip", address::zip);
    }

    #[test]
    fn country() {
        exec_mes("address::country", address::country);
    }

    #[test]
    fn country_abr() {
        exec_mes("address::country_abr", address::country_abr);
    }

    #[test]
    fn latitude() {
        exec_mes("address::latitude", || format!("{}", address::latitude()));
    }

    #[test]
    fn latitude_in_range() {
        exec_mes("address::latitude_in_range", || {
            format!("{}", address::latitude_in_range(-30.0, 30.0))
        });
    }

    #[test]
    fn longitude() {
        exec_mes("address::longitude", || format!("{}", address::longitude()));
    }

    #[test]
    fn longitude_in_range() {
        exec_mes("address::longitude_in_range", || {
            format!("{}", address::longitude_in_range(-30.0, 30.0))
        });
    }
}
