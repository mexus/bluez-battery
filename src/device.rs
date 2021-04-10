use std::fmt;

use anyhow::Result;
use dbus::arg::RefArg;
use regex::Regex;

use crate::get_string_property;

/// A device description.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Serialize)]
pub struct Device<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<&'a str>,
    address: &'a str,
}

impl<'a> Device<'a> {
    /// Creates a [Device] from a properties list.
    pub fn new(properties: &'a dbus::arg::PropMap) -> Result<Self> {
        let address = get_string_property(properties, "Address")?;
        let name = get_string_property(properties, "Name")?;
        let alias = properties.get("Alias").and_then(RefArg::as_str);
        Ok(Device {
            name,
            alias,
            address,
        })
    }

    /// Returns device name.
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// Returns either an alias (if exists) or a name of the device.
    pub fn display_name(&self) -> &'a str {
        self.alias.unwrap_or(self.name)
    }

    /// Returns `true` if the identifier match either name, alias or address of the device.
    pub fn matches(&self, identifier: &Regex) -> bool {
        identifier.is_match(self.name)
            || self
                .alias
                .map(|alias| identifier.is_match(alias))
                .unwrap_or(false)
            || identifier.is_match(self.address)
    }
}

impl fmt::Display for Device<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(alias) = &self.alias {
            if alias != &self.name {
                write!(f, "{} ({}, {})", alias, self.name, self.address)
            } else {
                write!(f, "{} ({})", alias, self.address)
            }
        } else {
            write!(f, "{} ({})", self.name, self.address)
        }
    }
}
