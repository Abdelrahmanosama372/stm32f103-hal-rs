
MEMORY 
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K 
  RAM : ORIGIN = 0x20000000, LENGTH = 20K 
}

ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS 
{
  .vector_table ORIGIN(FLASH) : 
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
    . = ALIGN(4) ;
  } > FLASH

  .rodata : 
  {
    *(.rodata .rodata.*);
    . = ALIGN(4) ;
  } > FLASH

  .text : 
  {
    *(.text .text.*);
    . = ALIGN(4) ;
    _etext = .;
  } > FLASH

  .data : AT(_etext)
  {
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4) ;
    _edata = .;
  } > RAM

  .bss : 
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4) ;
    _ebss = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }

  PROVIDE(NMI = DefaultExceptionHandler);
  PROVIDE(HardFault = DefaultExceptionHandler);
  PROVIDE(MemManage = DefaultExceptionHandler);
  PROVIDE(BusFault = DefaultExceptionHandler);
  PROVIDE(UsageFault = DefaultExceptionHandler);
  PROVIDE(SVCall = DefaultExceptionHandler);
  PROVIDE(PendSV = DefaultExceptionHandler);
  PROVIDE(SysTick = DefaultExceptionHandler);
}
