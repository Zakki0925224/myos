; https://os.phil-opp.com/multiboot-kernel/
; dd = define double (32bit)
; dw = define word (16bit)

section .multiboot_header
header_start:
    dd 0xe85250d6                   ; multiboot magic number
    dd 0                            ; architecture (0 = i386)
    dd header_end - header_start    ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
                                    ; checksum

    ; end tag (0, 0, 8)
    dw 0
    dw 0
    dd 8

header_end: