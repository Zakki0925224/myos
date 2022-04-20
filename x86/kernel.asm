global start

section .text
bits 32
start:
    ; call rust code
    extern kernel_main
    call kernel_main