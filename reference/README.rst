- The RISC-V Instruction Set Manual: Volume II: Privileged Architecture

  - riscv-privileged-20190608-1.pdf  
  - Document Version 20190608-Priv-MSU-Ratified
  - https://riscv.org/specifications/privileged-isa/

- The RISC-V Instruction Set Manual: Volume I: Unprivileged ISA

  - riscv-spec-20191213.pdf
  - Document Version 20191213
  - https://riscv.org/specifications/isa-spec-pdf/

- RISC-V from scratch

  - A guide on creating the _start function for the riscv virt machine.
  - https://twilco.github.io/riscv-from-scratch/2019/07/08/riscv-from-scratch-2.html

  - NOTES:

    - The source specifies ``.cfi_*`` llvm directives.
      These shouldn't be used for a _start function because the start function won't be called using the C foreign function interface.
      This _start function doesn't need to push the stack or anything..


- RISC-V assembly directives;

  - https://embarc.org/man-pages/as/RISC_002dV_002dDirectives.html
