extern crate bindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .trust_clang_mangling(false)
        .use_core()
        .ctypes_prefix("lang_items")

        .header("wrapper.h")

        .clang_arg("-I/opt/devkitpro/libnx/include")
        .clang_arg("-I/opt/devkitpro/devkitA64/aarch64-none-elf/include")
        
        .clang_arg("-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/7.3.0/include")
        .whitelist_function("gfxInitDefault")
        // .whitelist_function("consoleInit")
        .whitelist_function("consoleInit")
        .whitelist_function("appletMainLoop")
        .whitelist_function("hidScanInput")
        // .whitelist_function("hidKeysDown")
        .whitelist_function("hidKeysDown")
        .whitelist_function("gfxFlushBuffers")
        .whitelist_function("gfxSwapBuffers")
        .whitelist_function("gfxWaitForVsync")
        .whitelist_function("gfxExit")
        
        .whitelist_function("printf")

        // .blacklist_type("u16")
        // .blacklist_type("u32")
        // .blacklist_type("u64")
        .blacklist_type("u16")
        .blacklist_type("u32")
        .blacklist_type("u64")
        
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
