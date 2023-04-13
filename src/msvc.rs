/// MSVC
///
/// Windows build dependencies linked with vcpkg
///

#[cfg(target_env = "msvc")]
pub fn link_libraries() {
    #[link(name = "libmagic", kind = "static")]
    #[link(name = "curl", kind = "static")]
    #[link(name = "openssl", kind = "static")]
    #[link(name = "zlib", kind = "static")]
    extern "C" {}
}
