    .section .text
    .global _start
_start:
    cli
    lea stack_top(%rip), %rsp
    call kernel_main

halt_loop:
    hlt
    jmp halt_loop

    .section .bss
    .align 16
    .global stack_space
stack_space:
    .skip 0x4000
    .global stack_top
stack_top:
