[bits 16]           ; 16-bit real mode
[org 0x7c00]        ; BIOS loads bootloader at 0x7C00

start:
    mov si, message ; Load message address into SI register
    call print      ; Call print routine

input_loop:
    mov ah, 0
    int 0x16

    mov ah, 0x0e
    int 0x10

    jmp input_loop

print:
    mov ah, 0x0e    ; BIOS teletype output
.loop:
    lodsb           ; Load byte from SI into AL and increment SI
    cmp al, 0       ; Check if we've reached end of string
    je .done        ; If so, return
    int 0x10        ; Print character in AL
    jmp .loop       ; Repeat for next character
.done:
    ret

message: db 'DmitryOS: Type something', 13, 10,  0  ; Null-terminated string

times 510-($-$$) db 0   ; Pad with zeros
dw 0xaa55               ; Boot signature
