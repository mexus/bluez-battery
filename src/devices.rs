use std::collections::{hash_map::Iter as HashMapIter, HashMap};

use display_error_chain::DisplayErrorChain;
use regex::Regex;

use crate::{charge::BatteryCharge, Device, DeviceWithCharge};

/// An iterator over devices.
pub struct Devices<'a> {
    inner: HashMapIter<'a, dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>>,
    filter: Option<Regex>,
}

impl<'a> Devices<'a> {
    /// Creates an iterator over bluetooth devices with a battery from objects
    /// managed by the "org.bluez" d-bus service.
    pub fn new(
        objects: &'a HashMap<dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>>,
        filter: Option<Regex>,
    ) -> Self {
        Devices {
            inner: objects.iter(),
            filter,
        }
    }
}

impl<'a> Iterator for Devices<'a> {
    type Item = DeviceWithCharge<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (path, object) = self.inner.next()?;
            let device = match object.get("org.bluez.Device1").map(Device::new) {
                Some(Ok(device)) => device,
                Some(Err(e)) => {
                    log::debug!("Skipping {}: {}", path, DisplayErrorChain::new(&*e));
                    continue;
                }
                None => {
                    log::trace!(
                        r#"Skipping {} since it doesn't contain "org.bluez.Device1""#,
                        path
                    );
                    continue;
                }
            };
            if let Some(filter) = &self.filter {
                if !device.matches(filter) {
                    log::debug!(
                        r#"Skip {} since it doesn't match the filter "{}""#,
                        device,
                        filter
                    );
                    continue;
                }
            }
            let charge = match object.get("org.bluez.Battery1").map(BatteryCharge::new) {
                Some(Ok(charge)) => charge,
                Some(Err(e)) => {
                    log::warn!(
                        "[{}] Unable to extract charge: {}",
                        device,
                        DisplayErrorChain::new(&*e)
                    );
                    continue;
                }
                None => {
                    log::debug!(r#"[{}] No "org.bluez.Battery1" field"#, device);
                    continue;
                }
            };
            break Some(DeviceWithCharge { device, charge });
        }
    }
}
