
TARGET = armv7a-none-eabi
O = target
CONFIG = debug

all: bin

${O}:
	mkdir -p ${O}

ELF := ${O}/kernel.elf
BIN := ${O}/kernel.bin

CRT_0 := ${O}/crt0.o
LIBERTOS := $(O)/${TARGET}/${CONFIG}/libertos.rlib 

${ELF}: ${LIBERTOS} ${CRT_0}
	ld.lld -T src/asm/ld.lld $^ -o $@

${CRT_0}: | ${O}
	clang -target $(TARGET) -c src/asm/crt0.S -o ${CRT_0}

${LIBERTOS}: 
	xargo build "--target-dir=${O}" --target ${TARGET}

${BIN}: ${ELF}
	llvm-objcopy -O binary $^ $@

-include target/${TARGET}/${CONFIG}/libertos.d

elf: ${ELF}
bin: ${BIN}
clean:
	cargo clean

.PHONY: all clean elf bin
