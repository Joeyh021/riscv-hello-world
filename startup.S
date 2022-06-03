.section .text.start, "ax", @progbits
.globl _start
_start:
  li sp, 0x81000000 // end of boot memory
  call main
_hang:
  wfi
  j _hang
