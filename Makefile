all: bin

ARCH = riscv
TARGET = riscv64gc-none-elf

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
	qemu-system-riscv64 -M virt -bios none -kernel target/kernel.elf \
	    -m 128M -nographic -serial mon:stdio -gdb tcp::1234 -S
# Currently no lldb riscv support :(
#lldb:
#	lldb ${ELF} --one-line 'gdb-remote 1234' -s .lldbrc

gdb: 
	riscv64-linux-gnu-gdb target/kernel.elf --ex "target remote tcp::1234"
##########################################################################

.PHONY: all clean elf bin qemu lldb 
