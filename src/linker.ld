SECTIONS {
    . = 0x0000000080200000; 

    .text.boot ALIGN(4096): {  
        *(.text.boot)
    }

    .text ALIGN(4096): {  
        *(.text)
        *(.text.*)
    }

    .data ALIGN(4096): {  
        *(.data)
        *(.data.*)
    }

    .rodata ALIGN(4096): {  
        *(.rodata)
        *(.rodata.*)
    }

    .bss ALIGN(4096): {  
        _bss_start = .;
        *(.bss)
        *(.bss.*)
        . = ALIGN(4096);
        _stack_start = .;
        . += 0x10000;
        _stack_end = .;
        _bss_end = .;
    }

    heap_start = .;
} 

ENTRY(entrypoint)