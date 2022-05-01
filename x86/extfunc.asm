global init_sgm_reg

section .text

init_sgm_reg:
    jmp 0x08:.reload_cs

    .reload_cs:
        mov ax, 0x10
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov ss, ax
        ret