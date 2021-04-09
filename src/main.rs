use std::{process::exit, time::Duration};

use anyhow::{Context, Result};
use bluez_battery::Devices;
use dbus::blocking::{stdintf::org_freedesktop_dbus::ObjectManager, Connection};
use display_error_chain::DisplayErrorChain;
use structopt::StructOpt;

/// Extracts battery info from BlueZ daemon via D-Bus.
#[derive(Debug, StructOpt)]
struct Args {
    /// Activates debug output. Pass the flag twice (or more) to activate
    /// "trace" output.
    #[structopt(short, long, parse(from_occurrences))]
    debug: usize,

    /// Device name, alias or address to look for. Case insensitive, regular
    /// expressions supported. See https://docs.rs/regex/ for details.
    filter: Option<String>,
}

fn main() {
    let Args { debug, filter } = Args::from_args();
    if let Err(e) = bluez_battery::setup_logs(debug) {
        eprintln!("Unable to setup logger: {}", DisplayErrorChain::new(&e));
        exit(1);
    }
    if let Err(e) = run(filter) {
        log::error!("Terminating with error: {}", DisplayErrorChain::new(&*e));
        exit(1);
    }
}

fn run(filter: Option<String>) -> Result<()> {
    let connection =
        Connection::new_system().context("Unable to initialize a system dbus connection")?;
    log::trace!("Initialized connection {}", connection.unique_name());
    let proxy = connection.with_proxy("org.bluez", "/", Duration::from_secs(5));
    let objects = proxy
        .get_managed_objects()
        .context("Unable to get objects")?;
    log::trace!("Fetched objects:\n{:#?}", objects);
    let filter = filter
        .map(|filter| {
            regex::RegexBuilder::new(&filter)
                .case_insensitive(true)
                .build()
                .context("Unable to build a regular expression from filter")
        })
        .transpose()?;
    let devices = Devices::new(&objects, filter);
    for (device, charge) in devices {
        log::info!("{}: {}", device, charge);
    }
    Ok(())
}
