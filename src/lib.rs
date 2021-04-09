use anyhow::{Context, Result};
use dbus::arg::RefArg;

pub mod charge;
mod device;
mod devices;
mod logging;

pub use device::Device;
pub use devices::Devices;
pub use logging::setup_logs;

fn get_string_property<'a>(properties: &'a dbus::arg::PropMap, property: &str) -> Result<&'a str> {
    let value = properties
        .get(property)
        .with_context(|| format!(r#"No "{}" property"#, property))?;
    let value = value
        .as_str()
        .with_context(|| format!(r#""{}" is not a string: {:?}"#, property, value.arg_type()))?;
    Ok(value)
}
