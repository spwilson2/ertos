TARGET = riscv64gc-unknown-none-elf
O = target
CONFIG = debug

KERNEL := $(O)/$(TARGET)/$(CONFIG)/ertos

all: build
# Always build using cargo to enumerate rust dependencies.
_alwaysbuild:
${KERNEL}: _alwaysbuild
	cargo build "--target-dir=${O}"
${O}:
	mkdir -p ${O}
##########################################################################
# Rename additional targets
build: $(KERNEL)
##########################################################################
# Additional phony scripts
qemu: ${KERNEL}
	qemu-system-riscv64 -M virt -bios none -kernel ${KERNEL} \
	    -m 128M -nographic -serial mon:stdio -gdb tcp::1234 -S
gdb: 
	riscv64-linux-gnu-gdb ${KERNEL} --ex "target remote tcp::1234"
clean:
	cargo clean
##########################################################################

.PHONY: all clean build qemu
