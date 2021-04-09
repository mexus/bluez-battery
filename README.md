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

## License

License: Apache-2.0/MIT
