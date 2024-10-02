

#[cfg(feature = "rt")]
union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[cfg(feature = "rt")]
extern "C"
{
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick(); 
}

#[cfg(feature = "rt")]
#[allow(private_interfaces)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

/// A macro to define the main entry point of the application.
/// 
/// # Example
/// 
/// ```rust
/// entry!(main);
/// ```
/// 
/// The path provided (in this case `main`) should point to a function
/// with the signature `fn() -> !`.
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> ! {
            let f: fn() -> ! = $path;
            f();
        }
    }
}

/// The reset handler for the microcontroller, which initializes memory
/// (data and bss sections) and jumps to the `main` function.
///
/// This function is defined in the reset vector of the vector table and is
/// executed at startup.
#[cfg(feature = "rt")]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> !
{
    use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, write};

    extern "C" {
        static mut _sdata: u32;
        static mut _edata: u32;
        static mut _sbss: u32;
        static mut _ebss: u32;
        static mut _sidata: u32; 
    }

    let count = (addr_of!(_ebss) as *const u32).offset_from(addr_of!(_sbss) as *const u32);
    for i in 0..count {
        write((addr_of_mut!(_ebss) as *mut u32).offset(i as isize), 0);
    }
    
    let count = (addr_of!(_edata) as *const u32).offset_from(addr_of!(_sdata) as *const u32);
    copy_nonoverlapping(addr_of!(_sidata) as *const u32, addr_of_mut!(_sdata) as *mut u32, count as usize);
    extern "Rust" {
        fn main() -> !;
    } 

    main()
}

/// The reset vector for the microcontroller. This is the entry point to the system
/// after a reset, mapped in the `.vector_table.reset_vector` section.
#[cfg(feature = "rt")]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// A panic handler for when a panic occurs in the system. This is executed in case
/// of any unhandled error. It enters an infinite loop and halts the system.
#[cfg(feature = "rt")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> !
{
    loop {

    }
}

#[cfg(feature = "rt")]
#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

