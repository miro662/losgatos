SECTIONS {
    . = 0x0000000080200000; 

    .text.boot ALIGN(4096): {  
        *(.text.boot)
    }
} 

ENTRY(entrypoint)