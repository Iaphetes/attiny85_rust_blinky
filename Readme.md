# Rust on an ATTiny85

### Can it be?

## Requirements

The gcc compiler for AVR and arduino-avr-core libraries platforms.
Arch linux
```shell
$ pacman -S avr-gcc arduino-avr-core
```
avr-gcc

The exact Rust nightly toolchain described in [rust-toolchain.toml](rust-toolchain.toml)

It should automatically download when running the build command for the first time.

## Build command

```shell
$ build -Z build-std=core --target ./avr-unknown-gnu-attiny85.json --release
```

- Release is required!
- target is described in the target file
- the core library has to be built for the MCU as well


## Current progress

The WDT is working and a timer with a static sleep time is implemented

