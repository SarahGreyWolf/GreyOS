; Multiboot PDF https://nongnu.askapache.com/grub/phcoder/multiboot.pdf

header_start:
    dd 0xE85250D6                   ; Magic Number thing
    dd 0                            ; Architecture 0 for 32-bit protected mode of i386
    dd header_end - header_start
    ; Checksum, 32-bit unsigned value which when added to the other fields, must have a sum of 0
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; Multiboot tags go here, will probably want these later for framebuffer and stuff

    ; end tags - tells it that there are no more tags to be added
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size

header_end: