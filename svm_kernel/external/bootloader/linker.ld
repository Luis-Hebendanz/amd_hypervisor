ENTRY(_start_bootloader)

SECTIONS {
    . = 0x4000;
    . = ALIGN(0x1000);
    __smp_trampoline_start = .;
    .smp_trampoline :
    {
        *(.smp_trampoline)
    }
    . = ALIGN(0x1000);
    __smp_trampoline_end = .;
    . = 1M;
    __stack_guard = .;
    . += 0x1000;
    __stack_end = .;
    . += 0x6000;
    __stack_start = .;

    . = ALIGN(0x1000);
    __bootloader_start = .;
    .boot : ALIGN(2)
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
    }
    .bootloader : ALIGN(16)
    {
        *(.init_bootloader)
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.bss .bss.*)
        *(.got .got.*)
    }
    . = ALIGN(0x1000);
    __bootloader_end = .;

    . = 2M;
    __kernel_start = .;
    .kernel :
    {
        KEEP(*(.kernel))
    }
    . = ALIGN(0x1000);
    __kernel_end = .;

    . = ALIGN(0x1000);
    __page_table_start = .;
        _p4 = .;
        . += 0x1000;
        _p3 = .;
        . += 0x1000;
        _p2_tables_start = .;
            . = (8M - 0x2000);
        _p2_tables_end = .;
        . += 0x1000; /* First p1 table to map 0-2Mb with 4Kb pages*/
        _p1 = .;
    __page_table_end = .; /* Allocates 8Mb of RAM to map 4Gb of virt addr*/
    __minimum_mem_requirement = .;
}
