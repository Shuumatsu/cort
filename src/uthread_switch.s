.text
.globl uthread_switch
uthread_switch:
    mov     0x00[rdi], rsp
    mov     0x08[rdi], r15
    mov     0x10[rdi], r14
    mov     0x18[rdi], r13
    mov     0x20[rdi], r12
    mov     0x28[rdi], rbx
    mov     0x30[rdi], rbp


    mov     rsp, 0x00[rsi]
    mov     r15, 0x08[rsi]
    mov     r14, 0x10[rsi]
    mov     r13, 0x18[rsi]
    mov     r12, 0x20[rsi]
    mov     rbx, 0x28[rsi]
    mov     rbp, 0x30[rsi]

    ret
