# Arduino Pin Tracker

A CLI app to track, set and clear arduino pins and store the info in arduino_pins.json

## Usage

```bash
cargo run <COMMAND>
```

### Commands

- set-pin
- clear-pin
- status
- help

### Options

```
-h, --help Print help
-V, --version Print version
```

### Set Pin

Usage

```
cargo run set-pin <PIN> <FUNCTION> <MODE>
```

Example

```
cargo run set-pin 2 "Status LED" output
```
