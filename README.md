# BlueZ D-Bus battery charge extractor

A simple CLI tool that extracts battery charge information from a BlueZ daemon
via D-Bus.

## Usage

To obtain information about all the available devices, simply run the binary:

```
% cargo run
```

It will output a list of discovered devices and associated battery charge level,
like:

```
[INFO] WH-1000XM3 (38:18:4C:02:DE:F5): 80%
```

Run `cargo run -- --help` for more info.

## Machine-readable output

When built with feature `with_serde` (which is a default), a `-m, --machine`
flag becomes available. The flag makes the program to output information about
devices in a JSON format:

```
[{"name":"WH-1000XM3","alias":"LE_WH-1000XM3","address":"38:18:4C:02:DE:F5","charge":80}]
```

The output comes as an array of objects with the following fields:
* `name`: device name,
* `alias`: (optional) device alias,
* `address`: MAC address of the device,
* `charge`: the amount of battery left in percent (80 stands for 80%),
* `icon`: (optional) associated icon name.

## License

License: Apache-2.0/MIT
