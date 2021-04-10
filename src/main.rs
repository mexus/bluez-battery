use std::{process::exit, time::Duration};

use anyhow::{Context, Result};
use bluez_battery::{DeviceWithCharge, Devices};
use dbus::blocking::{stdintf::org_freedesktop_dbus::ObjectManager, Connection};
use display_error_chain::DisplayErrorChain;
use regex::{Regex, RegexBuilder};
use structopt::StructOpt;

fn make_regex(pattern: &str) -> Result<Regex, regex::Error> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}

/// Extracts battery info from BlueZ daemon via D-Bus.
#[derive(Debug, StructOpt)]
struct Args {
    /// Activates debug output. Pass the flag twice (or more) to activate
    /// "trace" output.
    #[structopt(short, long, parse(from_occurrences))]
    debug: usize,

    /// Produce machine-readable JSON output to stdout.
    #[cfg(feature = "with_serde")]
    #[structopt(short, long)]
    machine: bool,

    /// Device name, alias or address to look for. Case insensitive, regular
    /// expressions supported. See https://docs.rs/regex/ for details.
    #[structopt(parse(try_from_str = make_regex))]
    filter: Option<Regex>,
}

fn main() {
    let args = Args::from_args();
    if let Err(e) = bluez_battery::setup_logs(args.debug) {
        eprintln!("Unable to setup logger: {}", DisplayErrorChain::new(&e));
        exit(1);
    }
    if let Err(e) = run(args) {
        log::error!("Terminating with error: {}", DisplayErrorChain::new(&*e));
        exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let connection =
        Connection::new_system().context("Unable to initialize a system dbus connection")?;
    log::trace!("Initialized connection {}", connection.unique_name());
    let proxy = connection.with_proxy("org.bluez", "/", Duration::from_secs(5));
    let objects = proxy
        .get_managed_objects()
        .context("Unable to get objects")?;
    log::trace!("Fetched objects:\n{:#?}", objects);
    let devices = Devices::new(&objects, args.filter);
    #[cfg(feature = "with_serde")]
    if args.machine {
        let serialized = serde_json::to_string(&devices).context("Unable to serialize data")?;
        println!("{}", serialized);
        return Ok(());
    }
    for DeviceWithCharge { device, charge, .. } in devices {
        log::info!("{}: {}", device, charge);
    }

    Ok(())
}
