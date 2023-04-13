extern crate cc;
extern crate pkg_config;

fn main() {
    let zlib = pkg_config::probe_library("zlib").expect("zlib not found");
    let lzma = pkg_config::probe_library("lzma").expect("lzma not found");

    println!("cargo:rustc-link-lib=static=magic");
    println!(
        "cargo:rustc-link-search=native={}",
        zlib.link_paths[0].display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        lzma.link_paths[0].display()
    );
}
