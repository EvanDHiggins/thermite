extern rust_main

MBALIGN  equ  1<<0              ; align modules on page (4KiB) boundaries
MEMINFO  equ  1<<1              ; provide memory map
FLAGS    equ  MBALIGN | MEMINFO ; Multiboot flags
MAGIC    equ  0x1BADB002        ; Multiboot magic value
CHECKSUM equ  -(MAGIC + FLAGS)  ; CHECKSUM must sum to 0 with FLAGS and MAGIC

bits 32

section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM


section .bss
align 4096
p4_table:
resb 4096
p3_table:
resb 4096
p2_table:
resb 4096
stack_bottom:
resb 16384
stack_top:


section .text
global _start:function (_start.end - _start)
_start:

    ; Set up stack pointer
    mov esp, stack_top
    mov dword [0xb8000], 0x0145

    jmp long_mode_init

    hlt
.end:

long_mode_init:
    
    mov dword [0xb8002], 0x0F45
    mov dword [0xb8004], 0x0F44
    mov dword [0xb8006], 0x0F43
    mov dword [0xb8008], 0x0F42
    
    call detect_cpuid
    call detect_long_mode
    call enable_paging

    lgdt [gdt64.pointer]

    mov ax, gdt64.data

    ; stack segment
    mov ss, ax

    ; data segment
    mov ds, ax
    
    ; extra segment, not used.
    mov es, ax
    
    jmp gdt64.code:rust_main

    hlt


enable_paging:
    call init_page_tables
    
    mov eax, p4_table
    mov cr3, eax ; x86_64 expects address of P4 in CR3

    ; enable PAE
    mov eax, cr4
    or eax, 1<<5
    mov cr4, eax

    ; set long mode bit in model specific register (MSR)
    mov ecx, 0xC000_0080
    rdmsr
    or eax, 1<<8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31 ; Enable paging
    or eax, 1 << 16 ; Write protect
    mov cr0, eax

    ret


; Identity maps the first 512*2MiB = 1GiB of memory using
; the first entry of p4, p3, and huge pages in p2.
init_page_tables:
    ; Set first entry of p4 to point to p3
    mov eax, p3_table
    or eax, 0b11 ; Present + Writable
    mov dword [p4_table + 0], eax

    ; Set first entry of p3 to point to p2
    mov eax, p2_table
    or eax, 0b11 ; Present + Writable
    mov dword [p3_table + 0], eax

    mov ecx, 0
.init_p2_table:
    mov eax, 0x200_000
    mul ecx
    or eax, 0b10000011 ; HugePage + Present + Writable
    mov [p2_table + ecx*8], eax

    inc ecx
    cmp ecx, 512
    jne .init_p2_table

    ret
    

; To detect if long mode is supported we need to verify through CPUID.
; First, though, we need to verify that CPUID is available to us. This
; function does exactly that.
detect_cpuid:
    ; We know we have CPUID if we can flip the ID bit (bit 21) in the
    ; FLAGS register.

    ; Pushes the flags register onto the stack
    pushfd
    pop eax

    mov ecx, eax

    ; Flip bit in eax
    xor eax, 1<<21

    ; Push eax back into flags
    push eax
    popfd

    pushfd
    pop eax

    push ecx
    popfd

    cmp eax, ecx
    je .no_cpuid
    ret

.no_cpuid:
    mov eax, 1
    jmp error

; Checks if the extended functions which determine if long mode
; is available exist
detect_long_mode:
    mov eax, 0x80000000
    ; cpuid instr fills e{a,b,c,d}x with information based on the input
    ; value in eax
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    ; We now know that long mode is available to us
    mov eax, 0x80000001
    cpuid
    ; LM bit (bit 29) indicates long mode when high
    test edx, 1<<29
    jz .no_long_mode

    ret
    
.no_long_mode:
    mov eax, 2
    jmp error
    
; Prints errors through VGA buffer then halts.
; paramater: EAX: error code
error:
    mov dword [0xb8000], 0x0F45
    mov dword [0xb8002], 0x0F52
    mov dword [0xb8004], 0x0F52
    add eax, 30
    xor eax, 0x0F00
    mov dword [0xb8006], eax
    hlt
    
; Global Descriptor Table
; Not used in long mode, but it is still required to be set up
section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64

section .text
bits 64
long_mode_start:
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    hlt
