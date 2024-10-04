# STM32F103 HAL
STM32F103 HAL is a lightweight, no_std hardware abstraction layer (HAL) for the STM32F103 series of ARM Cortex-M microcontrollers. This crate is designed to simplify access to and control of STM32F103 peripherals while maintaining a strong focus on safety and ease of use.

## Features
- **No standard library**: Suitable for bare-metal and embedded environments, built with `#![no_std]`.
- **Runtime support**: Provides a reset handler, panic handler, and vector table.
- **GPIO abstraction**: Support for configuring and controlling GPIO pins with a simple and safe API.

### What's Coming Next?

i am actively working on expanding the support for additional STM32F103 peripherals. Here is the upcoming roadmap:

1. **EXTI (External Interrupts)**: Handle external interrupts for GPIO pins.
2. **UART**: Add support for serial communication (Universal Asynchronous Receiver-Transmitter).
3. **SPI**: Enable Serial Peripheral Interface support for communication with SPI-based devices.
4. **I2C**: Support for the Inter-Integrated Circuit (I2C) protocol.
5. **Timers**: Abstractions for working with hardware timers, including basic, general-purpose, and advanced timers.
6. **Other peripherals**: Future development will include ADC, DAC, CAN, PWM, etc.

## Quickstart

### 1. Add the crate to your `Cargo.toml`

First, you need to add the `stm32f103-hal` crate to your project. Add the following to your `Cargo.toml`:

```toml
[dependencies]
stm32f103-hal = { git = "https://github.com/Abdelrahmanosama372/stm32f103-hal-rs", features = ["rt"] }
```

### 2. Configure your project

Since this is a `no_std` project, you'll need to configure your project appropriately. Ensure your `main.rs` starts with:

```rust
#![no_std]
#![no_main]
```

### 3. Example: Blinking an LED on GPIO
A complete example that demonstrates configuring and toggling an LED on GPIO pin PA1 is provided in the examples directory.
To build the example:

```bash
cargo build --example gpio_example
```


### 4. Flashing and Running the Example
To build and flash this example onto your STM32F103 microcontroller in a single step, use cargo flash:

```bash
cargo flash --chip stm32f103c8 --example gpio_example
```
This command compiles the example and flashes the resulting binary to your microcontroller.

### 5. License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
