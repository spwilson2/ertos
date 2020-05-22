
TARGET = armv7a-none-eabi
O = target
CONFIG = debug

all: bin

${O}:
	mkdir -p ${O}

ELF := ${O}/kernel.elf
BIN := ${O}/kernel.bin

CRT_0 := ${O}/crt0.o
LIBERTOS := $(abspath $(O)/${TARGET}/${CONFIG}/libertos.rlib)

${ELF}: ${LIBERTOS} ${CRT_0}
	ld.lld -T src/asm/ld.lld $^ -o $@

${BIN}: ${ELF}
	llvm-objcopy -O binary $^ $@

${CRT_0}: | ${O}
	clang -target $(TARGET) -c src/asm/crt0.S -o ${CRT_0}



##########################################################################
### Cargo produces dependency files which we can use to track rebuilds ###
${LIBERTOS}: 
	xargo build "--target-dir=${O}" --target ${TARGET}
-include target/${TARGET}/${CONFIG}/libertos.d
##########################################################################

qemu:
	qemu-system-arm -M versatilepb -kernel target/kernel.bin \
	    -cpu cortex-a7 -m 128M -nographic -serial mon:stdio -gdb tcp::1234 -S
lldb:
	lldb ${ELF} --one-line 'gdb-remote 1234' -s .lldbrc

# Rename additional targets
elf: ${ELF}
bin: ${BIN}
clean:
	cargo clean

.PHONY: all clean elf bin
