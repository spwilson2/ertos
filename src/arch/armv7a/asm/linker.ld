ENTRY(start)
SECTIONS
{
    rust_eh_personality = .;
    . = 0x10000;
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    .bss :  { *(.bss)  }
    . = ALIGN(8);
    . = . + 0x1000;
    stack_top = .;
}
