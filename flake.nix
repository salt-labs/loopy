{
  description = "loopy";

  inputs = {
    nixpkgs = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-22.11";
      flake = true;
    };

    # https://devenv.sh/
    devenv = {
      type = "github";
      owner = "cachix";
      repo = "devenv";
      ref = "main";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      type = "github";
      owner = "nix-community";
      repo = "fenix";
      ref = "main";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    fenix,
    ...
  } @ inputs: let
    buildSystem = "x86_64-linux";

    hostSystems = [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];

    _targets = [
      "aarch64-apple-darwin"
      "aarch64-unknown-linux-gnu"
      "i686-unknown-linux-gnu"
      "x86_64-apple-darwin"
      "x86_64-unknown-linux-gnu"
    ];

    forAllSystems = f:
      builtins.listToAttrs (map (name: {
          inherit name;
          value = f name;
        })
        hostSystems);

    pkgsImportSystem = system:
      import nixpkgs {
        inherit system;
      };

    pkgsImportCrossSystem = buildPlatform: hostPlatform:
      if buildPlatform == hostPlatform
      then
        import inputs.nixpkgs {
          system = buildPlatform;
          localSystem = buildPlatform;
          crossSystem = buildPlatform;
        }
      else
        import inputs.nixpkgs {
          system = buildPlatform;
          localSystem = buildPlatform;
          crossSystem = hostPlatform;
        };

    _pkgsAllowUnfree = {
      nixpkgs = {
        config = {
          allowUnfree = true;
          allowUnfreePredicate = _: true;
        };
      };
    };
  in {
    packages = forAllSystems (hostPlatform: let
      # Build Platform
      system = buildSystem;
      inherit (self.packages.${system}) default;
      pkgs = pkgsImportSystem system;

      # Rust
      rustProfile = fenix.packages.${system}.complete;
      rustToolchain = rustProfile.toolchain;
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
      # Host Platform
      crossPkgs = pkgsImportCrossSystem system hostPlatform;
    in {
      loopy = import ./nix/packages/loopy {
        inherit pkgs;
        inherit crossPkgs;
        inherit rustPlatform;
      };

      default = self.packages.${system}.loopy;
    });

    devShells = forAllSystems (hostPlatform: let
      # Build Platform
      system = buildSystem;
      inherit (self.packages.${system}) default;
      pkgs = pkgsImportSystem system;

      # Rust
      rustProfile = fenix.packages.${system}.complete;
    in {
      devenv = import ./nix/devshells/devenv {
        inherit inputs;
        inherit pkgs;
        inherit rustProfile;
      };

      default = self.devShells.${system}.devenv;
    });
  };
}
