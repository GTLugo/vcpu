    .org $0000
ram_begin:
    
    .org $8000
    
reset:
    lda #$ff
    sta $6002

loop:
    lda #$55
    sta $6000

    lda #$aa
    sta $6000

    jmp loop

    .org $fffc
  
ram_end:
    .word reset
    .word $0000
    