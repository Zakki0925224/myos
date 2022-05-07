global start

section .text
bits 32
start:
    push ebx ; 1st argument of kernel_main
    push eax ; 2nd argument of kernel_main
    ; call rust code
    extern kernel_main
    call kernel_main