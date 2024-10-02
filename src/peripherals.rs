use core::{marker::PhantomData, ops::Deref};

pub mod rcc;
pub mod gpio;

use gpio::Gpio;

pub struct Rcc<'a> {
  _marker: PhantomData<&'a rcc::Rcc>  
}

impl<'a> Deref for Rcc<'a> 
{
    type Target = rcc::Rcc;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4002_1000 as *mut rcc::Rcc) 
        }
    }
}

pub struct GpioA<'a> {
  _marker: PhantomData<&'a Gpio>  
}

impl<'a> Deref for GpioA<'a> 
{
    type Target = Gpio;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4001_0800 as *mut Gpio) 
        }
    }
}

pub struct GpioB<'a> {
  _marker: PhantomData<&'a Gpio>  
}

impl<'a> Deref for GpioB<'a> 
{
    type Target = Gpio;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4001_0C00 as *mut Gpio) 
        }
    }
}

pub struct GpioC<'a> {
  _marker: PhantomData<&'a Gpio>  
}

impl<'a> Deref for GpioC<'a> 
{
    type Target = Gpio;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4001_1000 as *mut Gpio) 
        }
    }
}

pub struct GpioD<'a> {
  _marker: PhantomData<&'a Gpio>  
}

impl<'a> Deref for GpioD<'a> 
{
    type Target = Gpio;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4001_1400 as *mut Gpio) 
        }
    }
}

pub struct GpioE<'a> {
  _marker: PhantomData<&'a Gpio>  
}

impl<'a> Deref for GpioE<'a> 
{
    type Target = Gpio;
    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(0x4001_1800 as *mut Gpio) 
        }
    }
}

/// Struct containing all the peripherals for a given microcontroller, including:
/// * GPIO ports (A-E)
/// * Reset and Clock Control (RCC)
///
/// This struct provides easy access to the peripheral objects, allowing the user
/// to configure and control the microcontroller's peripherals.
///
/// # Example
/// ```
/// let peripherals = Peripherals::take();
/// let gpioa = peripherals.gpioa;
/// let rcc = peripherals.rcc;
/// ```
pub struct Peripherals<'a>
{
    pub gpioa: GpioA<'a>,
    pub gpiob: GpioB<'a>,
    pub gpioc: GpioC<'a>,
    pub gpiod: GpioD<'a>,
    pub gpioe: GpioE<'a>,

    pub rcc: Rcc<'a>
}

impl<'a> Peripherals<'a> 
{
    /// Acquires and returns the peripherals for the microcontroller.
    /// 
    /// This function returns a `Peripherals` instance, containing all the peripheral
    /// structs (GPIO ports, RCC, etc.), which can then be used to configure and control the hardware.
    ///
    /// # Example
    /// ```
    /// let peripherals = Peripherals::take();
    /// let gpioa = peripherals.gpioa;
    /// let rcc = peripherals.rcc;
    /// ```
    pub fn take() -> Self 
    {
        Peripherals 
        {
            gpioa: GpioA { _marker: PhantomData},
            gpiob: GpioB { _marker: PhantomData},
            gpioc: GpioC { _marker: PhantomData},
            gpiod: GpioD { _marker: PhantomData},
            gpioe: GpioE { _marker: PhantomData},

            rcc: Rcc {_marker: PhantomData},
        }
    }
}
