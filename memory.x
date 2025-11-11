MEMORY
{
    FLASH : ORIGIN = 0x10000000, LENGTH = 16M
    RAM   : ORIGIN = 0x20000000, LENGTH = 2M
}

SECTIONS
{
    .text : {
        KEEP(*(.vector_table))
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data : {
        *(.data*)
        . = ALIGN(4);
    } > RAM AT > FLASH

    .bss : {
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
    } > RAM
}
