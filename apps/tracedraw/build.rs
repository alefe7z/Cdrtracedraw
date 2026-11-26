// Embed the icon and version info into the Windows executable.
fn main() {
    println!("cargo:rerun-if-changed=../../assets/icon/tracedraw.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        // The main thread's stack, 1 MB on Windows by default, as large as
        // on Linux and macOS (8 MB): drawing a few hundred nested groups
        // overflowed it and ended the program (see THREAD_STACK in lib.rs).
        if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
            println!("cargo:rustc-link-arg-bin=tracedraw=/STACK:8388608");
        } else {
            println!("cargo:rustc-link-arg-bin=tracedraw=-Wl,--stack,8388608");
        }
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/icon/tracedraw.ico");
        res.set("ProductName", "CD Draw");
        res.set("FileDescription", "CD Draw vector editor");
        res.set("CompanyName", "Ighor Toth");
        res.set(
            "LegalCopyright",
            "Copyright \u{a9} 2026 TraceDraw members and contributors",
        );
        if let Err(e) = res.compile() {
            println!("cargo:warning=could not embed Windows resources: {e}");
        }
    }
}
