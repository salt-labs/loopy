{
  pkgs,
  crossPkgs,
  rustPlatform,
  ...
}:
rustPlatform.buildRustPackage {
  pname = "loopy";
  version = "0.1.0";

  src = ./../../../.;

  cargoLock.lockFile = ./../../../Cargo.lock;

  preBuild = "export TMPDIR=$(mktemp -d)";

  # buildPlatform
  nativeBuildInputs = with pkgs; [
    bzip2
    cargo
    clang
    cmake
    figlet
    file
    gcc
    gnutar
    openssl
    pkgconf
    rustc
    xxd
    zlib
    zstd
  ];

  # hostPlatform
  buildInputs = with pkgs; [
    bzip2
    figlet
    file
    gnutar
    openssl
    zstd
  ];

  doCheck = false;

  meta = {
    description = "loopy";
    homepage = "https://github.com/salt-labs/loopy";
    license = pkgs.lib.licenses.unlicense;
  };
}
