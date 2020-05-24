/*
 Linker script for outputting to RISC-V QEMU "virt" machine.
 Stephen Marz - 6 October 2019
*/

OUTPUT_ARCH( "riscv" )
ENTRY( _start )

/*
 * The MEMORY section will explain that we have "ram" that contains a section
 * that is 'w' (writeable), 'x' (executable), and 'a' (allocatable).  We use
 * '!' to invert 'r' (read-only) and 'i' (initialized). We don't want our
 * memory to be read-only, and we're stating that it is NOT initialized at the
 * beginning.
 *
 * The ORIGIN is the memory address 0x8000_0000. If we look at the virt spec or
 * the specification for the RISC-V HiFive Unleashed, this is the starting
 * memory address for our code.
 *
 * Side note: There might be other boot ROMs at different addresses, but their
 * job is to get to this point.
 *
 * Finally LENGTH = 128M tells the linker that we have 128 megabyte of RAM.
 * The linker will double check this to make sure everything can fit.
 *
 * The HiFive Unleashed has a lot more RAM than this, but for the virtual
 * machine, I went with 128M since I think that's enough RAM for now.
 *
 * We can provide other pieces of memory, such as QSPI, or ROM, but we're
 * telling the linker script here that we have one pool of RAM.
 */
MEMORY
{
  ram  (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/*
 * PHDRS is short for "program headers", which we specify three here:
 * text - CPU instructions (executable sections)
 * data - Global, initialized variables
 * bss  - Global, uninitialized variables (all will be set to 0 by boot.S)
 *
 * The command PT_LOAD tells the linker that these sections will be loaded
 * from the file into memory.
 *
 * We can actually stuff all of these into a single program header, but by
 * splitting it up into three, we can actually use the other PT_* commands
 * such as PT_DYNAMIC, PT_INTERP, PT_NULL to tell the linker where to find
 * additional information.
 *
 * However, for our purposes, every section will be loaded from the program
 * headers.
 */
PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  .text : {
          /*
            PROVIDE allows me to access a symbol called _text_start so
                I know where the text section starts in the operating system.
                This should not move, but it is here for convenience.
                The period '.' tells the linker to set _text_start to the
                CURRENT location ('.' = current memory location). This current
                memory location moves as we add things.
          */

    PROVIDE(_text_start = .);
    *(.text.init) *(.text .text.*)
    PROVIDE(_text_end = .);

    /*
     * The portion after the right brace is in an odd format. However, this is
     * telling the linker what memory portion to put it in. We labeled our
     * RAM, ram, with the constraints that it is writeable, allocatable, and
     * executable. The linker will make sure with this that we can do all of
     * those things.
     *
     * - >ram - This just tells the linker script to put this entire section
     *   (.text) into the ram region of memory. To my knowledge, the '>' does not
     *   mean "greater than". Instead, it is a symbol to let the linker know we
     *   want to put this in ram.
     *
     * - AT>ram - This sets the LMA (load memory address) region to the same
     *   thing. LMA is the final translation of a VMA (virtual memory address).
     *   With this linker script, we're loading everything into its physical
     *   location. We'll let the kernel copy and sort out the virtual memory.
     *   That's why >ram and AT>ram are continually the same thing.
     *
     * - :text  - This tells the linker script to put this into the :text
     *   program header. We've only defined three: text, data, and bss. In this
     *   case, we're telling the linker script to go into the text section.
     */
  } >ram AT>ram :text
   /*
    * The global pointer allows the linker to position global variables and
    * constants into independent positions relative to the gp (global pointer)
    * register. The globals start after the text sections and are only relevant
    * to the rodata, data, and bss sections.
   */
   PROVIDE(_global_pointer = .);

   .rodata : {
       PROVIDE(_rodata_start = .);
       *(.rodata .rodata.*)
       PROVIDE(_rodata_end = .);
  } >ram AT>ram :text

  .data : {
      . = ALIGN(4096);
      PROVIDE(_data_start = .);
      *(.sdata .sdata.*) *(.data .data.*)
      PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss : {
      PROVIDE(_bss_start = .);
      *(.sbss .sbss.*) *(.bss .bss.*)
      PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));

  /* Allocate space for the stack. */
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_end = _stack_start + 0x80000);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  /* Remainder is heap, don't bother aligning. The allocater will handle this. */
  PROVIDE(_heap_start = _stack_end);
  PROVIDE(_heap_size = _memory_end - _heap_start);
}
