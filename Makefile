CROSS_COMPILE = ~/project/vivado-risc-v/workspace/gcc/riscv/bin/riscv64-unknown-elf-

CC=$(CROSS_COMPILE)gcc
OBJCOPY=$(CROSS_COMPILE)objcopy
OBJDUMP=$(CROSS_COMPILE)objdump

CFLAGS = -march=rv64gc -mabi=lp64d -fno-builtin -ffreestanding

CCFLAGS = $(CFLAGS)
CCFLAGS += -mcmodel=medany -Wall -Werror
CCFLAGS += -fno-pic -fno-common -g -I.

LFLAGS = -static -nostartfiles -T main.lds


boot.elf: startup.S src/*.rs
	cargo build --release
	$(CC) $(CCFLAGS) $(LFLAGS) -o $@ startup.S target/riscv64gc-unknown-none-elf/release/librust.a
	$(OBJDUMP) -h -p $@

flash:
	cp boot.elf /media/joey/BOOT/boot.elf

clean:
	cargo clean
	rm -f *.elf
