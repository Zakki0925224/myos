global io_hlt, io_cli, io_sti, io_stihlt, io_test
section .text

io_hlt:
    hlt
    ret

io_cli:
    cli
    ret

io_sti:
    sti
    ret

io_stihlt:
    sti
    hlt
    ret

io_test:
    int 0x80