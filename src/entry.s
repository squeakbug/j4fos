.option norvc
.section .text._entry
.type _entry, @function
.global	_entry
_entry:

    /* Отключение всех прерываний */
    csrw sie, zero
	csrw sscratch, zero
	mv tp, zero

    csrw mideleg, zero
    csrw medeleg, zero
    csrw mie, zero
    csrw mip, zero

    la t5, stack0
    lui t6, 2
    add t5, t5, t6
    mv sp, t5

.option push
.option norelax
	la gp, __global_pointer$
.option pop

    csrw satp, zero

    la t5, bss_start
	la t6, bss_end
bss_clear:
	sd zero, (t5)
	addi t5, t5, 8
	bltu t5, t6, bss_clear

    la t0, kmain
	csrw mepc, t0

.extern kmain
    call kmain

finish:
    j finish

debug_message:
    .string "[DEBUG]\n\r"

.size _entry, . - _entry
stack0:
    .space 16384