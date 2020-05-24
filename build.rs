fn main() {
    use std::{env, path::PathBuf};
    use std::ffi::OsString;
    use std::fs;

    println!("cargo:rerun-if-changed=build.rs");

    if !env::var("TARGET").unwrap().contains("riscv64") {
      return;
    }

    let entries = fs::read_dir("src/arch/riscv64/asm/")
        .unwrap()
        .filter_map(|f| {
            f.ok().and_then(|e| {
                let path = e.path();
                match path.extension() {
                    Some(ext) if ext.eq(&OsString::from("S")) => Some(path),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();

    cc::Build::new()
        .compiler(PathBuf::from("clang"))
        .target("riscv64")
        .files(&entries)
        .pic(true)
        .static_flag(true)
        .shared_flag(false)
        .flag("-march=rv64gc")
        // Disable floating point code generation
        // https://gcc.gnu.org/onlinedocs/gcc/RISC-V-Options.html
        //  or ‘-march=rv64ifd -mabi=lp64’, in which no floating-point
        //  arguments will be passed in registers
        .flag("-mabi=lp64")
        .compile("asm");

    for e in entries {
        println!("cargo:rerun-if-changed={}", e.to_str().unwrap());
    }
}
