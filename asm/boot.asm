global start                        ; make label start public/global, entry point

section .text                       ; Default for executable code
bits 32                             ; Defines that the following are 32-bit instructions since we in protected mode
start:
    mov dword [0xb8000], 0x2f4b2f4f ; Move the double word constant 0x2f4b2f4f to memory address 0xb8000, prints OK
                                    ; Assuming 2f means to advanced the cursor by 1?
    hlt                             ; Cause the CPU to stop executing