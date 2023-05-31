{
  pkgs,
  crossPkgs,
  rustPlatform,
  ...
}:
rustPlatform.buildRustPackage {
  pname = "loopy";
  version = "0.2.0-beta.0";

  src = ./../../../.;

  #cargoRoot = ./..;

  cargoLock = {
    lockFile = ./../../../Cargo.lock;
  };

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./../../../Cargo.lock;
  };

  preBuild = ''
    echo "Running preBuild"

    export TMPDIR="$(mktemp -d)"

    cp ${./../../../Cargo.lock} Cargo.lock
  '';

  # buildPlatform
  nativeBuildInputs = with pkgs; [
    cargo
    rustc

    binutils
    bzip2
    clang
    cmake
    figlet
    file
    gcc
    gnutar
    lld
    glibc
    openssl
    perl
    pkgconf
    xxd
    zlib
    zstd
    xz
  ];

  # hostPlatform
  buildInputs = with pkgs; [
    bzip2
    figlet
    file
    gnutar
    openssl
    zlib
    zstd
  ];

  postPatch = ''
    echo "Running postPatch"

    cp ${./../../../Cargo.lock} Cargo.lock
  '';

  doCheck = false;

  meta = {
    description = "loopy";
    homepage = "https://github.com/salt-labs/loopy";
    license = pkgs.lib.licenses.unlicense;
  };
}
