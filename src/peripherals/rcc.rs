use volatile_register::RW;

#[allow(dead_code)]
pub struct Rcc 
{
    cr:       RW<u32>, 
    cfgr:     RW<u32>,
    cir:      RW<u32>,
    apb2rstr: RW<u32>,
    apb1rstr: RW<u32>,
    ahbenr:   RW<u32>,
    apb2enr:  RW<u32>,
    apb1enr:  RW<u32>,
    bdcr:     RW<u32>,
    csr:      RW<u32>
}


impl Rcc {

    #[inline(always)]
    pub fn enable_gpioa(&self) {
        unsafe { self.apb2enr.modify(|r| r | (1 << 2)) };
    }

    #[inline(always)]
    pub fn enable_gpiob(&self) {
        unsafe { self.apb2enr.modify(|r| r | (1 << 3)) };
    }

    #[inline(always)]
    pub fn enable_gpioc(&self) {
        unsafe { self.apb2enr.modify(|r| r | (1 << 4)) };
    }

    #[inline(always)]
    pub fn enable_gpiod(&self) {
        unsafe { self.apb2enr.modify(|r| r | (1 << 5)) };
    }

    #[inline(always)]
    pub fn enable_gpioe(&self) {
        unsafe { self.apb2enr.modify(|r| r | (1 << 6)) };
    }

}
