use std::fs;
use std::path::Path;

fn main() {
    let src_dir = Path::new("src");

    let mut asm_build = cc::Build::new();
    let mut c_build = cc::Build::new();
    let mut cpp_build = cc::Build::new();
    cpp_build.cpp(true);

    let mut has_asm = false;
    let mut has_c = false;
    let mut has_cpp = false;

    let entries = match fs::read_dir(src_dir) {
        Ok(entries) => entries,
        Err(err) => {
            println!("cargo:warning=failed to read src directory: {err}");
            return;
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                println!("cargo:warning=failed to read src entry: {err}");
                continue;
            }
        };
        if !path.is_file() {
            continue;
        }

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("s") => {
                asm_build.file(&path);
                has_asm = true;
            }
            Some("c") => {
                c_build.file(&path);
                has_c = true;
            }
            Some("cpp") => {
                cpp_build.file(&path);
                has_cpp = true;
            }
            _ => continue,
        }
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if has_asm {
        asm_build.compile("dev_asm");
    }
    if has_c {
        c_build.compile("dev_c");
    }
    if has_cpp {
        cpp_build.compile("dev_cpp");
    }
}
