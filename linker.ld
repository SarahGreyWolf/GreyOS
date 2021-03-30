/* Entry point of kernel, bootloader jumps to it after loading the kernel */
ENTRY(start)

SECTIONS {
    /* Sets the load address of the first section to 1MiB or 0x100000 
       Conventional place to load a kernel
    */
    . = 1M;
    /* First section of the executable */
    .boot :
    {
        /* Ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }
    
    /* Second section of the executable */
    .text :
    {
        /* Loads the .text section from boot.asm */
        *(.text)
    }
}
