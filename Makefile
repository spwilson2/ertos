all: bin

ARCH = armv7a
TARGET = arm-none-eabi

O = target
CONFIG = debug

BIN := ${O}/kernel.bin
LIBERTOS := $(abspath $(O)/${TARGET}/${CONFIG}/libertos.a)

include src/arch/$(ARCH)/Arch.mk

# Always build using cargo to enumerate rust dependencies.
_alwaysbuild:
${LIBERTOS}: _alwaysbuild
	xargo build "--target-dir=${O}" --target ${TARGET}
${BIN}: ${ELF}
	llvm-objcopy -O binary $^ $@
${O}:
	mkdir -p ${O}
##########################################################################
# Rename additional targets
elf: ${ELF}
bin: ${BIN}
clean:
	cargo clean
##########################################################################

##########################################################################
### 			Additional phony scripts 		       ###
qemu:
	qemu-system-arm -M versatilepb -kernel target/kernel.bin \
	    -cpu cortex-a7 -m 128M -nographic -serial mon:stdio -gdb tcp::1234 -S
lldb:
	lldb ${ELF} --one-line 'gdb-remote 1234' -s .lldbrc
##########################################################################

.PHONY: all clean elf bin qemu lldb 
