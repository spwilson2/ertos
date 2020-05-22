
TARGET = armv7a-none-eabi
O = target
CONFIG = debug

all: elf

${O}:
	mkdir -p ${O}

ELF := ${O}/kernel.elf
CRT_0 := ${O}/crt0.o
LIBERTOS := $(O)/${TARGET}/${CONFIG}/libertos.rlib 

${ELF}: ${LIBERTOS} ${CRT_0}
	ld.lld -T src/asm/ld.lld $^ -o $@

${CRT_0}: | ${O}
	clang -target $(TARGET) -c src/asm/crt0.S -o ${CRT_0}

${LIBERTOS}: 
	xargo build "--target-dir=${O}" --target ${TARGET}

-include target/${TARGET}/${CONFIG}/libertos.d

elf: ${ELF}
clean:
	cargo clean

.PHONY: all clean elf

