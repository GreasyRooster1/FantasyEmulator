; r1 = ballx
; r2 = bally
; r3 = ballx dir
; r4 = bally dir
; r5 = screen w
; r6 = screen h

; bounce x 0x0000
BRANCH r3 == r0 0 12
BRANCH r3 != r0 0 0x00
; 12
LODI r3 0_1

;LODI r3 0_1