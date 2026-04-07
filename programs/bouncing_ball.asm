; r0 = 1
; r1 = ballx
; r2 = bally
; r3 = ballx dir
; r4 = bally dir
; r5 = screen w
; r6 = screen h

LODI r0 0_1

; bounce x  0100
BRANCH r3 == r0 1 12
BRANCH r3 != r0 1 16
; 12
LODI r3 0_1
; 16
LODI r3 0_0

; bounce y 0200
BRANCH r4 == r0 2 12
BRANCH r4 != r0 2 16
; 12
LODI r4 0_1
; 16
LODI r4 0_0