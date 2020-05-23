ENTRY(start)
SECTIONS
{
    rust_eh_personality = .;
    . = 0x80000000;
    .text : { *(.text.init),  *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    .bss :  { *(.bss)  }

    # Allocate space for the stack
    . = ALIGN(8);
    . = . + 0x1000;
    __stack_top = .;
}
