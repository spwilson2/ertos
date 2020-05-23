
# The arch file provides the following targets:
# $(ELF) $(RUST_TARGET_PATH)/$(target).json
this := $(abspath $(lastword $(MAKEFILE_LIST)))
path := $(shell dirname $(this))
export RUST_TARGET_PATH := $(path)

CRT_0 := $(O)/crt0.o
ELF := $(O)/kernel.elf

CC_OPTS := -ggdb --target=riscv64

ELF_DEPS := $(CRT_0) $(LIBERTOS) 

$(ELF): $(path)/asm/linker.ld $(ELF_DEPS)
	ld.lld --gc-sections -T $< -o $@ $(ELF_DEPS)

$(CRT_0): $(path)/asm/crt0.S $(this) | $(O)
	clang $(CC_OPTS) -mno-relax -fpic -c $< -o $(CRT_0)

elf: $(ELF)
