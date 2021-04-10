use std::{convert::TryInto, fmt};

use anyhow::{Context, Result};
use dbus::arg::{ArgType, RefArg};

/// Battery charge (in percent).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize))]
#[cfg_attr(feature = "with_serde", serde(transparent))]
pub struct BatteryCharge {
    value: u8,
}

impl fmt::Display for BatteryCharge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2}%", self.value)
    }
}

impl BatteryCharge {
    /// Extracts a battery charge percentage from a properties map.
    pub fn new(properties: &dbus::arg::PropMap) -> Result<BatteryCharge> {
        let percentage = properties
            .get("Percentage")
            .context(r#"No "Percentage" field"#)?;
        anyhow::ensure!(
            matches!(percentage.arg_type(), ArgType::Variant),
            r#"Unexpected "Percentage" kind: {:?}"#,
            percentage.arg_type()
        );
        let percentage = percentage
            .as_iter()
            .expect("Variant is iterable")
            .next()
            .context(r#"Empty "Percentage" value"#)?;
        let percentage = percentage.as_u64().with_context(|| {
            format!(
                r#""Percentage" is wrapped in an unexpected value kind: {:?}"#,
                percentage.arg_type()
            )
        })?;
        let value = percentage
            .try_into()
            .with_context(|| format!(r#"Invalid "Percentage" value: {}"#, percentage))?;
        Ok(BatteryCharge { value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(" 1%", BatteryCharge { value: 1 }.to_string());
        assert_eq!("99%", BatteryCharge { value: 99 }.to_string());
        assert_eq!("100%", BatteryCharge { value: 100 }.to_string());
    }

    #[cfg(feature = "with_serde")]
    #[test]
    fn serialize() {
        serde_test::assert_ser_tokens(&BatteryCharge { value: 50 }, &[serde_test::Token::U8(50)]);
    }
}
