//! Helper types and functions.

#![deny(missing_docs)]

use anyhow::{Context, Result};
use dbus::arg::RefArg;

mod charge;
mod device;
mod devices;
mod logging;

pub use charge::BatteryCharge;
pub use device::Device;
pub use devices::Devices;
pub use logging::setup_logs;

/// Device with a charge.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, serde::Serialize)]
pub struct DeviceWithCharge<'a> {
    /// Device.
    #[serde(flatten)]
    pub device: Device<'a>,

    /// Battery charge.
    pub charge: BatteryCharge,
}

/// Extracts a property value as a rust string.
fn get_string_property<'a>(properties: &'a dbus::arg::PropMap, property: &str) -> Result<&'a str> {
    let value = properties
        .get(property)
        .with_context(|| format!(r#"No "{}" property"#, property))?;
    let value = value
        .as_str()
        .with_context(|| format!(r#""{}" is not a string: {:?}"#, property, value.arg_type()))?;
    Ok(value)
}
