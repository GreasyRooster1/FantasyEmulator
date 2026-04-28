; r0 = 1
; r1 = ballx
; r2 = bally
; r3 = ballx dir (0 for left, 1 for right)
; r4 = bally dir
; r5 = screen w
; r6 = screen h


LODI r0 0_0_0_1

.bounce_x
BREQ r3 r0 .bounce_x_left
BREZ r3 .bounce_x_right
.bounce_x_right
LODI r3 0_0_0_1
.bounce_x_left
LODI r3 0_0_0_0

.bounce_y
BREQ r4 r0 .bounce_y_left
BREZ r4 .bounce_y_right
.bounce_y_right
LODI r4 0_0_0_1
.bounce_y_left
LODI r4 0_0_0_0

.move_ball
BREQ r3 r0 .inc_x
BREZ r3 .dec_x
BREQ r4 r0 .inc_y
BREZ r4 .dec_y
.inc_x
ADDI r1 0_0_0_1
.dec_x
SUBI r1 0_0_0_1
.inc_y
ADDI r2 0_0_0_1
.dec_y
SUBI r2 0_0_0_1

.draw_ball


