[bits 16] ;16
[org 0x7c00]

start:
    ; Set video mode (320x200, 256 colors)
    mov ax, 0x13
    int 0x10

    ; Draw circle
    mov cx, 169    ; Center X (screen middle)
    mov dx, 121    ; Center Y (screen middle)
    mov bx, 60     ; Radius
    mov al, 13     ; Color (white)
    call draw_circle

    ; Wait for key
    mov ah, 0
    int 0x16

    ; Return to text mode
    mov ax, 0x3
    int 0x10

    ; Halt
    jmp $

; Draw circle using midpoint circle algorithm
; Parameters:
;   CX = center X
;   DX = center Y
;   BX = radius
;   AL = color
draw_circle:
    push bp
    mov bp, sp
    sub sp, 6
    mov [bp-2], cx    ; Store center X
    mov [bp-4], dx    ; Store center Y
    mov [bp-6], bx    ; Store radius

    xor si, si        ; x = 0
    mov di, bx        ; y = radius
    mov bx, 1
    sub bx, di        ; d = 1 - radius

.loop:
    cmp si, di
    jg .done

    ; Plot 8 points
    call plot_circle_points

    inc si            ; x++

    ; Update decision variable
    cmp bx, 0
    jl .skip_y
    dec di            ; y--
    add bx, si
    add bx, si
    sub bx, di
    sub bx, di
    add bx, 1
    jmp .continue

.skip_y:
    add bx, si
    add bx, si
    add bx, 1

.continue:
    jmp .loop

.done:
    mov sp, bp
    pop bp
    ret

; Plot 8 symmetrical points of the circle
; Parameters same as draw_circle
plot_circle_points:
    push ax
    push cx
    push dx

    mov cx, [bp-2]    ; center X
    mov dx, [bp-4]    ; center Y

    ; Plot (x,y)
    push cx
    push dx
    add cx, si
    add dx, di
    call plot_pixel
    pop dx
    pop cx

    ; Plot (x,-y)
    push cx
    push dx
    add cx, si
    sub dx, di
    call plot_pixel
    pop dx
    pop cx

    ; Plot (-x,y)
    push cx
    push dx
    sub cx, si
    add dx, di
    call plot_pixel
    pop dx
    pop cx

    ; Plot (-x,-y)
    push cx
    push dx
    sub cx, si
    sub dx, di
    call plot_pixel
    pop dx
    pop cx

    ; Plot (y,x)
    push cx
    push dx
    add cx, di
    add dx, si
    call plot_pixel
    pop dx
    pop cx

    ; Plot (y,-x)
    push cx
    push dx
    add cx, di
    sub dx, si
    call plot_pixel
    pop dx
    pop cx

    ; Plot (-y,x)
    push cx
    push dx
    sub cx, di
    add dx, si
    call plot_pixel
    pop dx
    pop cx

    ; Plot (-y,-x)
    push cx
    push dx
    sub cx, di
    sub dx, si
    call plot_pixel
    pop dx
    pop cx

    pop dx
    pop cx
    pop ax
    ret

; Plot a single pixel
; Parameters:
;   CX = X coordinate
;   DX = Y coordinate
;   AL = color
plot_pixel:
    push es
    push bx

    mov bx, 0xa000
    mov es, bx

    ; Calculate pixel offset (y * 320 + x)
    push ax
    mov ax, 320
    mul dx
    add ax, cx
    mov bx, ax
    pop ax

    mov [es:bx], al

    pop bx
    pop es
    ret

times 510-($-$$) db 0
dw 0xaa55
