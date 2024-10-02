#![no_std]

//! # STM32F103 HAL (Hardware Abstraction Layer)
//! 
//! This crate provides a Hardware Abstraction Layer (HAL) for the STM32F103 series of microcontrollers.
//! It offers abstractions for working with GPIO and peripherals in a safe and convenient way.
//! 
//! ## Features
//! - GPIO Pin control
//! - Peripherals abstraction
//! - Runtime setup with reset handlers

/// Peripherals module containing abstractions for working with STM32F103 peripherals.
pub mod peripherals;

/// Runtime (rt) module for system initialization, reset handling, and interrupt vectors.
/// This module includes the reset handler and vector table initialization.
pub mod rt;

