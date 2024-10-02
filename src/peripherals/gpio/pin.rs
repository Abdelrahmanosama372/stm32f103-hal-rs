use core::marker::PhantomData;

use super::Gpio;

pub struct Disabled;
pub struct Locked;

pub trait PinState {}
pub trait OutputState {}
pub trait InputState {}

pub struct PushPull;
pub struct OpenDrain;

impl OutputState for PushPull {}
impl OutputState for OpenDrain {}

impl PinState for PushPull {}
impl PinState for OpenDrain {}

pub struct Analog;
pub struct Floating; 
pub struct InputPullUp; 
pub struct InputPullDown; 

impl InputState for Analog {}
impl InputState for Floating {}
impl InputState for InputPullUp {}
impl InputState for InputPullDown {}

impl PinState for Analog {}
impl PinState for Floating {}
impl PinState for InputPullUp {}
impl PinState for InputPullDown {}

/*
    01: Output mode, max speed 10 MHz.
    10: Output mode, max speed 2 MHz.
    11: Output mode, max speed 50 MHz.
*/
pub enum PinSpeed 
{
    Speed2Hz = 0b10,
    Speed10Hz = 0b01,
    Speed50Hz = 0b11,
}

pub struct PinConfig<'a,const N: u8, STATE>
{
    gpio: &'a Gpio,
    _state: core::marker::PhantomData<STATE>,
}

/// Creates a new `PinConfig` for a specified GPIO pin in a disabled state.
/// 
/// # Arguments
/// * `gpio` - Reference to the `Gpio` struct representing the GPIO port.
///
/// # Returns
/// A new `PinConfig` object for pin `N` in a `Disabled` state.
impl <'a, const N:u8, STATE> PinConfig<'a, N, STATE>
{
    pub fn new(gpio: &'a Gpio) -> PinConfig<N, Disabled>
    {
        PinConfig::<N, Disabled>{
            gpio,
            _state: PhantomData,
        }
    }
}

/// Configures the pin as an output in push-pull mode with a specified speed.
/// 
/// # Arguments
/// * `pin_speed` - The speed mode for the output pin (`Speed2Hz`, `Speed10Hz`, `Speed50Hz`).
/// 
/// # Returns
/// A new `PinConfig` object for pin `N` in `PushPull` output state.
///
/// # Example
/// ```
/// let pin = gpio.p0().into_output_pushpull(PinSpeed::Speed50Hz);
/// ```
impl<'a, const N:u8> PinConfig<'a, N, Disabled>
{
   pub fn into_output_pushpull(&self, pin_speed:PinSpeed) -> PinConfig<'a, N, PushPull>
   {   
        let cnf = 0b00;
        let mode = pin_speed as u32;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }
        PinConfig::<N, PushPull> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   }

   /// Configures the pin as an output in open-drain mode with a specified speed.
   /// 
   /// # Arguments
   /// * `pin_speed` - The speed mode for the output pin (`Speed2Hz`, `Speed10Hz`, `Speed50Hz`).
   /// 
   /// # Returns
   /// A new `PinConfig` object for pin `N` in `OpenDrain` output state.
   ///
   /// # Example
   /// ```
   /// let pin = gpio.p1().into_output_opendrain(PinSpeed::Speed2Hz);
   /// ```
   pub fn into_output_opendrain(&self, pin_speed:PinSpeed) -> PinConfig<'a, N, OpenDrain>
   {    
        let cnf = 0b01;
        let mode = pin_speed as u32;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }
        PinConfig::<N, OpenDrain> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   } 

   /// Configures the pin as an analog input.
   /// 
   /// # Returns
   /// A new `PinConfig` object for pin `N` in `Analog` input state.
   ///
   /// # Example
   /// ```
   /// let pin = gpio.p2().into_analog();
   /// ```
   pub fn into_analog(&self) -> PinConfig<'a, N, Analog>
   {    
        let cnf = 0b00;
        let mode = 0b00;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }
        PinConfig::<N, Analog> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   }

   /// Configures the pin as an input with a floating state (no pull-up or pull-down resistor).
   /// 
   /// # Returns
   /// A new `PinConfig` object for pin `N` in `Floating` input state.
   ///
   /// # Example
   /// ```
   /// let pin = gpio.p3().into_input_floating();
   /// ```
   pub fn into_input_floating(&self) -> PinConfig<'a, N, Floating>
   {    
        let cnf = 0b01;
        let mode = 0b00;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }
        PinConfig::<N, Floating> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   } 

   /// Configures the pin as an input with an internal pull-up resistor.
   /// 
   /// # Returns
   /// A new `PinConfig` object for pin `N` in `InputPullUp` state.
   ///
   /// # Example
   /// ```
   /// let pin = gpio.p4().into_input_pullup();
   /// ```
   pub fn into_input_pullup(&self) -> PinConfig<'a, N, InputPullUp>
   {    
        let cnf = 0b10;
        let mode = 0b00;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }

        unsafe {
            self.gpio.odr.modify(|r| r | (1 << N));
        }

        PinConfig::<N, InputPullUp> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   } 

   /// Configures the pin as an input with an internal pull-down resistor.
   /// 
   /// # Returns
   /// A new `PinConfig` object for pin `N` in `InputPullDown` state.
   ///
   /// # Example
   /// ```
   /// let pin = gpio.p5().into_input_pulldown();
   /// ```
   pub fn into_input_pulldown(&self) -> PinConfig<'a, N, InputPullDown>
   {    
        let cnf = 0b10;
        let mode = 0b00;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }

        unsafe {
            self.gpio.odr.modify(|r| r & !(1 << N));
        }

        PinConfig::<N, InputPullDown> {
            gpio: self.gpio, 
            _state: PhantomData,
        }
   } 
}


impl<'a, const N:u8, STATE: OutputState> PinConfig<'a, N, STATE>
{
    /// Sets the pin to a high state (logical 1).
    /// 
    /// This only works for pins configured as outputs. The function writes to the BSRR register.
    /// 
    /// # Example
    /// ```
    /// let pin = gpio.p6().into_output_pushpull(PinSpeed::Speed50Hz);
    /// pin.set();
    /// ```
    pub fn set(&self) 
    {
        unsafe {
          self.gpio.bsrr.write(1 << N); 
        }    
    }

    /// Resets the pin to a low state (logical 0).
    /// 
    /// This only works for pins configured as outputs. The function writes to the BRR register.
    /// 
    /// # Example
    /// ```
    /// let pin = gpio.p7().into_output_pushpull(PinSpeed::Speed50Hz);
    /// pin.reset();
    /// ```
    pub fn reset(&self) 
    {
        unsafe {
          self.gpio.brr.write(1 << N); 
        }    
    }
}

impl<'a, const N:u8, STATE: InputState> PinConfig<'a, N, STATE>
{
    /// Checks if the input pin is in a high state (logical 1).
    /// 
    /// This function reads from the IDR register to determine the pin state.
    /// 
    /// # Returns
    /// `true` if the pin is high, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let pin = gpio.p8().into_input_floating();
    /// if pin.is_high() {
    ///     // Do something
    /// }
    /// ```
    pub fn is_high(&self) -> bool
    {
        let r = self.gpio.idr.read(); 
        (r & (1 << N)) != 0
    }

    /// Checks if the input pin is in a low state (logical 0).
    /// 
    /// This function reads from the IDR register to determine the pin state.
    /// 
    /// # Returns
    /// `true` if the pin is low, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let pin = gpio.p9().into_input_floating();
    /// if pin.is_low() {
    ///     // Do something
    /// }
    /// ```
    pub fn is_low(&self) -> bool
    {
        let r = self.gpio.idr.read(); 
        (r & (1 << N)) == 0
    }
}

impl<'a, const N: u8, STATE: PinState> PinConfig<'a, N, STATE>
{
    /// Locks the configuration of the pin, preventing further modifications.
    /// 
    /// This follows the lock sequence as described in the hardware manual, which requires a series of writes and reads to the LCKR register.
    /// 
    /// # Returns
    /// A new `PinConfig` object for pin `N` in the `Locked` state.
    ///
    /// # Example
    /// ```
    /// let pin = gpio.p10().into_output_pushpull(PinSpeed::Speed50Hz);
    /// let locked_pin = pin.lock();
    /// ```
    pub fn lock(&self) -> PinConfig<'a, N, Locked>
    {
        unsafe {
           let f = self.gpio.lckr.read() | 1;
           self.gpio.lckr.write(f);
           //self.gpio.lckr.modify(|r| r | (1 << 0)); 
        }
        /*
         * lock sequence:
         * -------------
         * write 1 
         * write 0 
         * write 1 
         * read 0 
         * read 1 (this read is optional but confirms that the lock is active)
         */
        unsafe {
            self.gpio.lckr.modify(|r| r | (1 << 16)); 
            self.gpio.lckr.modify(|r| r & !(1 << 16)); 
            self.gpio.lckr.modify(|r| r | (1 << 16)); 
            self.gpio.lckr.read(); 
            self.gpio.lckr.read(); 
        }

        PinConfig::<N, Locked>
        {
            gpio: self.gpio,
            _state: PhantomData,
        }
    }

    /// Disables the pin, returning it to a default state.
    /// 
    /// # Returns
    /// A new `PinConfig` object for pin `N` in the `Disabled` state.
    ///
    /// # Example
    /// ```
    /// let pin = gpio.p11().into_output_pushpull(PinSpeed::Speed50Hz);
    /// let disabled_pin = pin.disable();
    /// ```
    pub fn disable(&self) -> PinConfig<'a, N, Disabled>
    {
        let cnf = 0b01;
        let mode = 0b00;
        let bits = (cnf << 2) | mode;

        match N 
        {
            0..=7 => {
                unsafe {    
                    self.gpio.crl.modify(|r|{
                        let offset = N * 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }
            }
            8..=15 => {
                unsafe {    
                    self.gpio.crh.modify(|r|{
                        let offset = (N - 8)* 4;
                        r & !(0b1111 << offset) | (bits << offset)
                    })
                }

            }
            _ => ()
        }

        unsafe 
        {
            self.gpio.brr.write(1 << N);
        }

        PinConfig::<N, Disabled>
        {
            gpio: self.gpio,
            _state: PhantomData,
        }
    }

}

