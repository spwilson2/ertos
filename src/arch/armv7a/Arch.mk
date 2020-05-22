
# The arch file provides the following targets:
# $(ELF) $(RUST_TARGET_PATH)/$(target).json
this := $(abspath $(lastword $(MAKEFILE_LIST)))
path := $(shell dirname $(this))
export RUST_TARGET_PATH := $(path)

CRT_0 := $(O)/crt0.o
ELF := $(O)/kernel.elf

ELF_DEPS := $(CRT_0) $(LIBERTOS) 

$(ELF): $(path)/asm/linker.ld $(ELF_DEPS)
	ld.lld -T $< -o $@ $(ELF_DEPS)

$(CRT_0): $(this) | $(O)
	clang -target $(TARGET) -c $(path)/asm/crt0.S -o $(CRT_0)

elf: $(ELF)
