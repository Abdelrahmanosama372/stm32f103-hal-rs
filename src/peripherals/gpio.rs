
use pin::{Disabled, PinConfig};
use volatile_register::*;

pub mod pin;

pub struct Gpio
{
    pub crl: RW<u32>,
    pub crh: RW<u32>,
    pub idr: RO<u32>,
    pub odr: RW<u32>,
    pub bsrr: WO<u32>,
    pub brr: WO<u32>,
    pub lckr: RW<u32>,
}

macro_rules! pin {
    ($px: ident, $num: literal) => (
        pub fn $px(&self) -> PinConfig<$num, Disabled> 
        {
            PinConfig::<$num, Disabled>::new(self)
        }
    )
}

impl Gpio {
   pin!(p0, 0);
   pin!(p1, 1);
   pin!(p2, 2);
   pin!(p3, 3);
   pin!(p4, 4);
   pin!(p5, 5);
   pin!(p6, 6);
   pin!(p7, 7);
   pin!(p8, 8);
   pin!(p9, 9);
   pin!(p10, 10);
   pin!(p11, 11);
   pin!(p12, 12);
   pin!(p13, 13);
   pin!(p14, 14);
   pin!(p15, 15);
}

pub use pin::PinSpeed; 
