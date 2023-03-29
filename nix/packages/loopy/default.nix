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

  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    pkgconf
    cmake

    bzip2
    figlet
    file
    gnutar
    openssl
    xxd
    zlib
    zstd
  ];

  buildInputs = with pkgs; [
    bzip2
    figlet
    file
    gnutar
    openssl
    xxd
    zlib
    zstd
  ];

  doCheck = false;

  meta = {
    description = "loopy";
    homepage = "https://github.com/salt-labs/loopy";
    license = pkgs.lib.licenses.unlicense;
  };
}
