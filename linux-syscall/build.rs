use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=src/syscall.h.in");

    let mut fout = std::fs::File::create("src/consts.rs").unwrap();
    writeln!(fout, "// Generated by build.rs. DO NOT EDIT.").unwrap();
    writeln!(fout, "use numeric_enum_macro::numeric_enum;\n").unwrap();
    writeln!(fout, "numeric_enum! {{").unwrap();
    writeln!(fout, "#[repr(u32)]").unwrap();
    writeln!(fout, "#[derive(Debug, Eq, PartialEq)]").unwrap();
    writeln!(fout, "#[allow(non_camel_case_types)]").unwrap();
    writeln!(fout, "pub enum SyscallType {{").unwrap();

    let data = std::fs::read_to_string("src/syscall.h.in").unwrap();
    for line in data.lines() {
        if !line.starts_with("#define") {
            continue;
        }
        let mut iter = line.split_whitespace();
        let _ = iter.next().unwrap();
        let name = iter.next().unwrap();
        let id = iter.next().unwrap();

        let name = &name[5..].to_uppercase();
        writeln!(fout, "    {} = {},", name, id).unwrap();
    }
    writeln!(fout, "}}").unwrap();
    writeln!(fout, "}}").unwrap();
}
