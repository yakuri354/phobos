ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFFE000000000;

SECTIONS {
    . = KERNEL_OFFSET;

    . += SIZEOF_HEADERS;
    . = ALIGN(4096);

    .text : AT(ADDR(.text) - KERNEL_OFFSET) {
        *(.text*)
        . = ALIGN(4096);
    }

    .rodata : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata*)
        . = ALIGN(4096);
    }

    .data : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data* .got)
        . = ALIGN(4096);
        *(.bss*)
        . = ALIGN(4096);
    }

    .tdata : AT(ADDR(.tdata) - KERNEL_OFFSET) {
        *(.tdata*)
        . = ALIGN(4096);
        *(.tbss*)
        . += 8;
        . = ALIGN(4096);
    }

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}
