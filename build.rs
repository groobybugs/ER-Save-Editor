use std::{env, io};

fn main() -> io::Result<()> {
    // Only Windows targets carry an embedded icon/resource.
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        use winres::WindowsResource;
        let host_is_windows = env::var("HOST").unwrap_or_default().contains("windows");
        let mut res = WindowsResource::new();
        // This path can be absolute, or relative to your crate root.
        res.set_icon("./icon/icon.ico");
        if !host_is_windows {
            // Cross-compiling: plain `windres`/`ar` target the host, so point
            // winres at the mingw-w64 cross binaries matching the target arch.
            let prefix = if env::var("TARGET").unwrap_or_default().starts_with("i686") {
                "i686-w64-mingw32"
            } else {
                "x86_64-w64-mingw32"
            };
            res.set_windres_path(&format!("{prefix}-windres"));
            res.set_ar_path(&format!("{prefix}-ar"));
        }
        // On non-Windows hosts the mingw-w64 tools may be missing (or the target
        // is msvc); build the exe anyway without the icon instead of failing.
        match res.compile() {
            Ok(()) => {
                if !host_is_windows {
                    // GNU ld only pulls archive members that resolve a symbol, so the
                    // resource-only member of libresource.a gets dropped. Feed the
                    // object file straight to the linker instead.
                    println!(
                        "cargo:rustc-link-arg-bins={}/resource.o",
                        env::var("OUT_DIR").unwrap()
                    );
                }
            }
            Err(err) => {
                if host_is_windows {
                    return Err(err);
                }
                println!("cargo:warning=skipping icon resource embedding: {err}");
            }
        }
    }
    Ok(())
}
